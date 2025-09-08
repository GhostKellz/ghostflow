-- Seed data for GhostFlow

-- Insert default project
INSERT INTO projects (id, name, description, created_by) VALUES 
    ('00000000-0000-0000-0000-000000000001', 'Default Project', 'Default project for flows', 'system');

-- Insert system user
INSERT INTO users (id, username, email, full_name) VALUES 
    ('00000000-0000-0000-0000-000000000001', 'system', 'system@ghostflow.local', 'System User');

-- Insert sample flow
INSERT INTO flows (id, name, description, definition, created_by, tags, category) VALUES 
    ('00000000-0000-0000-0000-000000000001', 
     'Hello World Flow',
     'A simple hello world flow for testing',
     '{
        "id": "00000000-0000-0000-0000-000000000001",
        "name": "Hello World Flow",
        "description": "A simple hello world flow for testing",
        "version": "1.0.0",
        "nodes": {
            "start": {
                "id": "start",
                "node_type": "http_request",
                "name": "Hello Request",
                "description": "Makes a simple HTTP request",
                "parameters": {
                    "method": "GET",
                    "url": "https://httpbin.org/json"
                },
                "position": {
                    "x": 100,
                    "y": 100
                },
                "timeout_ms": 30000
            }
        },
        "edges": [],
        "triggers": [
            {
                "id": "manual_trigger",
                "trigger_type": {
                    "type": "manual"
                },
                "config": {},
                "enabled": true
            }
        ],
        "parameters": {},
        "secrets": [],
        "metadata": {
            "created_at": "2025-01-01T00:00:00Z",
            "updated_at": "2025-01-01T00:00:00Z",
            "created_by": "system",
            "tags": ["example", "http"],
            "category": "example"
        }
     }',
     'system',
     ARRAY['example', 'http'],
     'example'
    );

-- Insert some example secrets (for development only)
INSERT INTO secrets (key, value, description, created_by) VALUES 
    ('example_api_key', 'sk_test_123456789', 'Example API key for testing', 'system'),
    ('database_url', 'postgresql://localhost/ghostflow', 'Database connection URL', 'system');