-- Initial GhostFlow database schema

-- Flows table
CREATE TABLE flows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) NOT NULL DEFAULT '1.0.0',
    definition JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    tags TEXT[] DEFAULT '{}',
    category VARCHAR(100),
    enabled BOOLEAN NOT NULL DEFAULT true
);

-- Flow executions table
CREATE TABLE flow_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_id UUID NOT NULL REFERENCES flows(id) ON DELETE CASCADE,
    flow_version VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled', 'retrying')),
    trigger_type VARCHAR(50) NOT NULL,
    trigger_source VARCHAR(255),
    trigger_metadata JSONB DEFAULT '{}',
    input_data JSONB,
    output_data JSONB,
    error_type VARCHAR(50),
    error_message TEXT,
    error_details JSONB,
    error_retryable BOOLEAN,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    execution_time_ms BIGINT,
    executor_id VARCHAR(255) NOT NULL DEFAULT 'default',
    environment VARCHAR(100) NOT NULL DEFAULT 'local',
    correlation_id VARCHAR(255),
    trace_id VARCHAR(255),
    span_id VARCHAR(255)
);

-- Node executions table
CREATE TABLE node_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_execution_id UUID NOT NULL REFERENCES flow_executions(id) ON DELETE CASCADE,
    node_id VARCHAR(255) NOT NULL,
    node_type VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'failed', 'cancelled', 'retrying')),
    input_data JSONB,
    output_data JSONB,
    error_type VARCHAR(50),
    error_message TEXT,
    error_details JSONB,
    error_retryable BOOLEAN,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    execution_time_ms BIGINT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

-- Execution logs table
CREATE TABLE execution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_execution_id UUID NOT NULL REFERENCES flow_executions(id) ON DELETE CASCADE,
    node_execution_id UUID REFERENCES node_executions(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    level VARCHAR(10) NOT NULL CHECK (level IN ('TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR')),
    message TEXT NOT NULL,
    details JSONB
);

-- Secrets table (for development - in production use proper vault)
CREATE TABLE secrets (
    key VARCHAR(255) PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL DEFAULT 'system',
    description TEXT
);

-- Artifacts table
CREATE TABLE artifacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flow_execution_id UUID NOT NULL REFERENCES flow_executions(id) ON DELETE CASCADE,
    node_execution_id UUID REFERENCES node_executions(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    checksum VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Projects table (for future multi-tenancy)
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL DEFAULT 'system'
);

-- Users table (for future auth)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(320) NOT NULL UNIQUE,
    full_name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    active BOOLEAN NOT NULL DEFAULT true
);

-- Create indexes for better query performance
CREATE INDEX idx_flows_name ON flows(name);
CREATE INDEX idx_flows_category ON flows(category);
CREATE INDEX idx_flows_created_at ON flows(created_at);
CREATE INDEX idx_flows_tags ON flows USING GIN(tags);

CREATE INDEX idx_flow_executions_flow_id ON flow_executions(flow_id);
CREATE INDEX idx_flow_executions_status ON flow_executions(status);
CREATE INDEX idx_flow_executions_started_at ON flow_executions(started_at);
CREATE INDEX idx_flow_executions_trigger_type ON flow_executions(trigger_type);

CREATE INDEX idx_node_executions_flow_execution_id ON node_executions(flow_execution_id);
CREATE INDEX idx_node_executions_node_id ON node_executions(node_id);
CREATE INDEX idx_node_executions_status ON node_executions(status);

CREATE INDEX idx_execution_logs_flow_execution_id ON execution_logs(flow_execution_id);
CREATE INDEX idx_execution_logs_timestamp ON execution_logs(timestamp);
CREATE INDEX idx_execution_logs_level ON execution_logs(level);

CREATE INDEX idx_artifacts_flow_execution_id ON artifacts(flow_execution_id);
CREATE INDEX idx_artifacts_node_execution_id ON artifacts(node_execution_id);

-- Add a trigger to automatically update the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_flows_updated_at
    BEFORE UPDATE ON flows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_secrets_updated_at
    BEFORE UPDATE ON secrets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();