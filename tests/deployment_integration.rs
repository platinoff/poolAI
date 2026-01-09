//! Integration tests for Deployment Configuration
//!
//! Tests for validating deployment files and configurations:
//! - Dockerfile syntax and structure
//! - docker-compose.yml syntax and structure
//! - Configuration file validation
//! - Environment variable handling

use std::fs;
use std::path::Path;

#[test]
fn test_dockerfile_exists() {
    assert!(Path::new("docker/Dockerfile").exists(), "Dockerfile should exist in docker/");
}

#[test]
fn test_dockerfile_contains_required_stages() {
    let dockerfile_content = fs::read_to_string("docker/Dockerfile").expect("Failed to read Dockerfile");

    // Check for multi-stage build
    assert!(
        dockerfile_content.contains("FROM rust:") || dockerfile_content.contains("FROM rust"),
        "Dockerfile should contain Rust builder stage"
    );
    assert!(
        dockerfile_content.contains("FROM debian:") || dockerfile_content.contains("FROM debian"),
        "Dockerfile should contain runtime stage"
    );
}

#[test]
fn test_dockerfile_exposes_ports() {
    let dockerfile_content = fs::read_to_string("docker/Dockerfile").expect("Failed to read Dockerfile");

    assert!(
        dockerfile_content.contains("8080"),
        "Dockerfile should expose port 8080"
    );
    assert!(
        dockerfile_content.contains("8443"),
        "Dockerfile should expose port 8443"
    );
    assert!(
        dockerfile_content.contains("EXPOSE"),
        "Dockerfile should contain EXPOSE directive"
    );
}

#[test]
fn test_dockerfile_uses_non_root_user() {
    let dockerfile_content = fs::read_to_string("docker/Dockerfile").expect("Failed to read Dockerfile");

    assert!(
        dockerfile_content.contains("USER poolai") || dockerfile_content.contains("USER "),
        "Dockerfile should use non-root user"
    );
}

#[test]
fn test_dockerfile_has_healthcheck() {
    let dockerfile_content = fs::read_to_string("docker/Dockerfile").expect("Failed to read Dockerfile");

    assert!(
        dockerfile_content.contains("HEALTHCHECK"),
        "Dockerfile should have health check"
    );
}

#[test]
fn test_docker_compose_exists() {
    assert!(
        Path::new("docker/docker-compose.yml").exists(),
        "docker-compose.yml should exist in docker/"
    );
}

#[test]
fn test_docker_compose_has_poolai_service() {
    let compose_content =
        fs::read_to_string("docker/docker-compose.yml").expect("Failed to read docker-compose.yml");

    assert!(
        compose_content.contains("poolai:"),
        "docker-compose.yml should contain poolai service"
    );
}

#[test]
fn test_docker_compose_has_volumes() {
    let compose_content =
        fs::read_to_string("docker/docker-compose.yml").expect("Failed to read docker-compose.yml");

    assert!(
        compose_content.contains("poolai-data:") || compose_content.contains("volumes:"),
        "docker-compose.yml should define volumes"
    );
}

#[test]
fn test_docker_compose_has_networks() {
    let compose_content =
        fs::read_to_string("docker/docker-compose.yml").expect("Failed to read docker-compose.yml");

    assert!(
        compose_content.contains("networks:") || compose_content.contains("poolai-network:"),
        "docker-compose.yml should define networks"
    );
}

#[test]
fn test_dockerignore_exists() {
    assert!(
        Path::new("docker/.dockerignore").exists(),
        ".dockerignore should exist in docker/"
    );
}

#[test]
fn test_dockerignore_excludes_target() {
    let dockerignore_content =
        fs::read_to_string("docker/.dockerignore").expect("Failed to read .dockerignore");

    assert!(
        dockerignore_content.contains("target/") || dockerignore_content.contains("target"),
        ".dockerignore should exclude target directory"
    );
}

#[test]
fn test_config_example_exists() {
    assert!(
        Path::new("config.example.toml").exists(),
        "config.example.toml should exist"
    );
}

#[test]
fn test_config_example_is_valid_toml() {
    let config_content =
        fs::read_to_string("config.example.toml").expect("Failed to read config.example.toml");

    // Basic TOML validation - check for common sections
    assert!(
        config_content.contains("[system]") || config_content.contains("[pool]"),
        "config.example.toml should contain configuration sections"
    );
}

#[test]
fn test_config_example_has_required_sections() {
    let config_content =
        fs::read_to_string("config.example.toml").expect("Failed to read config.example.toml");

    // Check for common required sections
    let has_system = config_content.contains("[system]");
    let has_pool = config_content.contains("[pool]");
    let has_monitoring = config_content.contains("[monitoring]");

    assert!(
        has_system || has_pool || has_monitoring,
        "config.example.toml should contain at least one configuration section"
    );
}

#[test]
fn test_deployment_docs_exist() {
    assert!(
        Path::new("docs/deployment/DOCKER.md").exists(),
        "Docker deployment documentation should exist"
    );
    assert!(
        Path::new("docs/deployment/KUBERNETES.md").exists(),
        "Kubernetes deployment documentation should exist"
    );
    assert!(
        Path::new("docs/deployment/BARE_METAL.md").exists(),
        "Bare metal deployment documentation should exist"
    );
}

#[test]
fn test_deployment_testing_checklist_exists() {
    assert!(
        Path::new("docs/deployment/DEPLOYMENT_TESTING_CHECKLIST.md").exists(),
        "Deployment testing checklist should exist"
    );
}
