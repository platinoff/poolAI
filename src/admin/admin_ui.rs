use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use super::pool_cok::{PoolNode, MigrationTask};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUI {
    api_url: String,
}

impl AdminUI {
    pub fn new(api_url: String) -> Self {
        Self { api_url }
    }

    pub async fn serve_index(&self) -> impl Responder {
        let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Pool Admin Panel</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 20px; }
                .container { max-width: 1200px; margin: 0 auto; }
                .card { border: 1px solid #ddd; padding: 20px; margin: 10px 0; border-radius: 5px; }
                .btn { padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 5px; cursor: pointer; }
                .btn-danger { background: #dc3545; }
                table { width: 100%; border-collapse: collapse; }
                th, td { padding: 10px; border: 1px solid #ddd; text-align: left; }
                .form-group { margin: 10px 0; }
                input, select { padding: 8px; width: 100%; }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Pool Admin Panel</h1>
                
                <div class="card">
                    <h2>Nodes</h2>
                    <table id="nodes-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>URL</th>
                                <th>Capacity</th>
                                <th>Current Load</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody id="nodes-body"></tbody>
                    </table>
                    <button class="btn" onclick="showAddNodeForm()">Add Node</button>
                </div>

                <div class="card">
                    <h2>Migrations</h2>
                    <table id="migrations-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>Source</th>
                                <th>Target</th>
                                <th>Priority</th>
                                <th>Status</th>
                            </tr>
                        </thead>
                        <tbody id="migrations-body"></tbody>
                    </table>
                    <button class="btn" onclick="showForceMigrationForm()">Force Migration</button>
                </div>
            </div>

            <script>
                let sessionId = null;

                async function login() {
                    const token = prompt('Enter admin token:');
                    const response = await fetch('/login', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ token })
                    });
                    if (response.ok) {
                        const data = await response.json();
                        sessionId = data.session_id;
                        loadData();
                    } else {
                        alert('Login failed');
                    }
                }

                async function loadData() {
                    if (!sessionId) {
                        login();
                        return;
                    }

                    // Load nodes
                    const nodesResponse = await fetch('/nodes', {
                        headers: { 'Authorization': sessionId }
                    });
                    const nodes = await nodesResponse.json();
                    const nodesBody = document.getElementById('nodes-body');
                    nodesBody.innerHTML = nodes.map(node => `
                        <tr>
                            <td>${node.id}</td>
                            <td>${node.url}</td>
                            <td>${node.capacity}</td>
                            <td>${node.current_load}</td>
                            <td>
                                <button class="btn btn-danger" onclick="removeNode('${node.id}')">Remove</button>
                            </td>
                        </tr>
                    `).join('');

                    // Load migrations
                    const migrationsResponse = await fetch('/migrations', {
                        headers: { 'Authorization': sessionId }
                    });
                    const migrations = await migrationsResponse.json();
                    const migrationsBody = document.getElementById('migrations-body');
                    migrationsBody.innerHTML = migrations.map(migration => `
                        <tr>
                            <td>${migration.id}</td>
                            <td>${migration.source_node}</td>
                            <td>${migration.target_node}</td>
                            <td>${migration.priority}</td>
                            <td>${migration.status}</td>
                        </tr>
                    `).join('');
                }

                async function addNode(node) {
                    const response = await fetch('/nodes', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'Authorization': sessionId
                        },
                        body: JSON.stringify(node)
                    });
                    if (response.ok) {
                        loadData();
                    }
                }

                async function removeNode(nodeId) {
                    const response = await fetch(`/nodes/${nodeId}`, {
                        method: 'DELETE',
                        headers: { 'Authorization': sessionId }
                    });
                    if (response.ok) {
                        loadData();
                    }
                }

                async function forceMigration(task) {
                    const response = await fetch('/migrations/force', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'Authorization': sessionId
                        },
                        body: JSON.stringify(task)
                    });
                    if (response.ok) {
                        loadData();
                    }
                }

                function showAddNodeForm() {
                    const node = {
                        id: prompt('Node ID:'),
                        url: prompt('Node URL:'),
                        capacity: parseInt(prompt('Capacity:')),
                        current_load: 0
                    };
                    addNode(node);
                }

                function showForceMigrationForm() {
                    const task = {
                        source_node: prompt('Source Node ID:'),
                        target_node: prompt('Target Node ID:'),
                        priority: parseInt(prompt('Priority:')),
                        task_data: {}
                    };
                    forceMigration(task);
                }

                // Initial load
                login();
            </script>
        </body>
        </html>
        "#;

        HttpResponse::Ok()
            .content_type("text/html")
            .body(html)
    }
} 