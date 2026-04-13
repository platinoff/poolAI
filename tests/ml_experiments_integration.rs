//! Integration tests for Experiment Tracking Module (ML.5)
//!
//! Tests the experiment tracking functionality including registration,
//! metrics logging, status management, and best experiment selection.

#[cfg(not(feature = "ml"))]
#[test]
fn ml_tests_skipped_no_ml_feature() {}

#[cfg(feature = "ml")]
mod tests {
    use poolai::ml::experiments::{
        Experiment, ExperimentMetrics, ExperimentStatus, ExperimentTracker,
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_tracker_creation() {
        let tracker = ExperimentTracker::new();
        let list = tracker.list_experiments(None).await;
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_start_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker
            .start_experiment("exp1", "NeuralNetwork")
            .await
            .unwrap();
        assert_eq!(exp.name, "exp1");
        assert_eq!(exp.model_type, "NeuralNetwork");
        assert_eq!(exp.status, ExperimentStatus::Running);
    }

    #[tokio::test]
    async fn test_start_experiment_empty_name() {
        let tracker = ExperimentTracker::new();
        let result = tracker.start_experiment("", "NeuralNetwork").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_metrics() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        let mut metrics = ExperimentMetrics::default();
        metrics.accuracy = 0.95;
        metrics.loss = 0.05;
        metrics.training_time_ms = 1000;

        tracker.log_metrics(exp.id.as_str(), metrics).await.unwrap();

        let got: Experiment = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert!(got.metrics.is_some());
        let m = got.metrics.unwrap();
        assert!((m.accuracy - 0.95).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_end_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        tracker.end_experiment(exp.id.as_str()).await.unwrap();

        let got: Experiment = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.status, ExperimentStatus::Completed);
        assert!(got.ended_at.is_some());
    }

    #[tokio::test]
    async fn test_fail_experiment() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        tracker.fail_experiment(exp.id.as_str()).await.unwrap();

        let got: Experiment = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.status, ExperimentStatus::Failed);
    }

    #[tokio::test]
    async fn test_get_best_by_accuracy() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        let mut m1 = ExperimentMetrics::default();
        m1.accuracy = 0.9;
        tracker.log_metrics(e1.id.as_str(), m1).await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        let mut m2 = ExperimentMetrics::default();
        m2.accuracy = 0.95;
        tracker.log_metrics(e2.id.as_str(), m2).await.unwrap();
        tracker.end_experiment(e2.id.as_str()).await.unwrap();

        let best: Experiment = tracker.get_best_by_accuracy().await.unwrap();
        assert!((best.metrics.unwrap().accuracy - 0.95).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_get_best_by_loss() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        let mut m1 = ExperimentMetrics::default();
        m1.loss = 0.1;
        tracker.log_metrics(e1.id.as_str(), m1).await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        let mut m2 = ExperimentMetrics::default();
        m2.loss = 0.05;
        tracker.log_metrics(e2.id.as_str(), m2).await.unwrap();
        tracker.end_experiment(e2.id.as_str()).await.unwrap();

        let best: Experiment = tracker.get_best_by_loss().await.unwrap();
        assert!((best.metrics.unwrap().loss - 0.05).abs() < 1e-9);
    }

    #[tokio::test]
    async fn test_list_experiments_filtered() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let _e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        // e2 still running

        let completed = tracker
            .list_experiments(Some(ExperimentStatus::Completed))
            .await;
        assert_eq!(completed.len(), 1);

        let running = tracker
            .list_experiments(Some(ExperimentStatus::Running))
            .await;
        assert_eq!(running.len(), 1);
    }

    #[tokio::test]
    async fn test_add_hyperparameters_and_tags() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        let mut params = HashMap::new();
        params.insert("lr".to_string(), "0.001".to_string());
        tracker
            .add_hyperparameters(exp.id.as_str(), params)
            .await
            .unwrap();

        tracker
            .add_tags(exp.id.as_str(), vec!["best".to_string()])
            .await
            .unwrap();

        let got: Experiment = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        assert_eq!(got.hyperparameters.get("lr"), Some(&"0.001".to_string()));
        assert!(got.tags.contains(&"best".to_string()));
    }

    #[tokio::test]
    async fn test_compare_by_accuracy() {
        let tracker = ExperimentTracker::new();

        let e1 = tracker.start_experiment("e1", "NN").await.unwrap();
        let mut m1 = ExperimentMetrics::default();
        m1.accuracy = 0.9;
        tracker.log_metrics(e1.id.as_str(), m1).await.unwrap();
        tracker.end_experiment(e1.id.as_str()).await.unwrap();

        let e2 = tracker.start_experiment("e2", "NN").await.unwrap();
        let mut m2 = ExperimentMetrics::default();
        m2.accuracy = 0.95;
        tracker.log_metrics(e2.id.as_str(), m2).await.unwrap();
        tracker.end_experiment(e2.id.as_str()).await.unwrap();

        let ord = tracker
            .compare_by_accuracy(e1.id.as_str(), e2.id.as_str())
            .await
            .unwrap();
        assert_eq!(ord, std::cmp::Ordering::Less);
    }

    #[tokio::test]
    async fn test_custom_metrics() {
        let tracker = ExperimentTracker::new();
        let exp = tracker.start_experiment("exp1", "NN").await.unwrap();

        let mut metrics = ExperimentMetrics::default();
        metrics.accuracy = 0.95;
        metrics.custom.insert("f1_score".to_string(), 0.92);
        metrics.custom.insert("precision".to_string(), 0.94);

        tracker.log_metrics(exp.id.as_str(), metrics).await.unwrap();

        let got: Experiment = tracker.get_experiment(exp.id.as_str()).await.unwrap();
        let m = got.metrics.unwrap();
        assert!((m.custom.get("f1_score").unwrap() - 0.92).abs() < 1e-9);
        assert!((m.custom.get("precision").unwrap() - 0.94).abs() < 1e-9);
    }
}
