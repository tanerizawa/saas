-- Migration for onboarding workflow, system configuration, and email tracking
-- Created: 2025-07-29 for Phase 4B Database Integration

-- Onboarding workflow table
CREATE TABLE IF NOT EXISTS onboarding_workflows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
    current_step INTEGER NOT NULL DEFAULT 1,
    total_steps INTEGER NOT NULL DEFAULT 7,
    completion_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    status VARCHAR(50) NOT NULL DEFAULT 'in_progress',
    estimated_completion_time INTEGER, -- in minutes
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Onboarding steps tracking
CREATE TABLE IF NOT EXISTS onboarding_steps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES onboarding_workflows(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    step_name VARCHAR(100) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, in_progress, completed, skipped
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    data JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(workflow_id, step_number)
);

-- License processing workflow table
CREATE TABLE IF NOT EXISTS license_processing_workflows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    license_id UUID NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,
    current_stage INTEGER NOT NULL DEFAULT 1,
    total_stages INTEGER NOT NULL DEFAULT 8,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal', -- urgent, high, normal, low
    assigned_reviewer_id UUID REFERENCES users(id),
    processing_notes TEXT,
    escalated BOOLEAN NOT NULL DEFAULT FALSE,
    escalation_reason TEXT,
    sla_deadline TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- License processing stages tracking
CREATE TABLE IF NOT EXISTS license_processing_stages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES license_processing_workflows(id) ON DELETE CASCADE,
    stage_number INTEGER NOT NULL,
    stage_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, in_progress, completed, failed
    reviewer_id UUID REFERENCES users(id),
    review_action VARCHAR(50), -- approve, reject, request_revision, escalate
    review_comments TEXT,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    data JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(workflow_id, stage_number)
);

-- System configuration groups and settings
CREATE TABLE IF NOT EXISTS system_config_groups (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS system_config_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    group_id UUID NOT NULL REFERENCES system_config_groups(id) ON DELETE CASCADE,
    key VARCHAR(255) NOT NULL,
    value TEXT,
    default_value TEXT,
    data_type VARCHAR(50) NOT NULL DEFAULT 'string', -- string, integer, boolean, json
    description TEXT,
    validation_rule TEXT, -- JSON schema or regex pattern
    is_sensitive BOOLEAN NOT NULL DEFAULT FALSE,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(group_id, key)
);

-- Email tracking and templates
CREATE TABLE IF NOT EXISTS email_templates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    subject VARCHAR(255) NOT NULL,
    html_body TEXT NOT NULL,
    text_body TEXT,
    variables JSONB DEFAULT '[]', -- List of available variables
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS email_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    template_id UUID REFERENCES email_templates(id),
    recipient_email VARCHAR(255) NOT NULL,
    recipient_name VARCHAR(255),
    subject VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, sent, failed, bounced
    error_message TEXT,
    sent_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_onboarding_workflows_user_id ON onboarding_workflows(user_id);
CREATE INDEX IF NOT EXISTS idx_onboarding_workflows_status ON onboarding_workflows(status);
CREATE INDEX IF NOT EXISTS idx_onboarding_steps_workflow_id ON onboarding_steps(workflow_id);

CREATE INDEX IF NOT EXISTS idx_license_processing_workflows_license_id ON license_processing_workflows(license_id);
CREATE INDEX IF NOT EXISTS idx_license_processing_workflows_reviewer ON license_processing_workflows(assigned_reviewer_id);
CREATE INDEX IF NOT EXISTS idx_license_processing_workflows_priority ON license_processing_workflows(priority);
CREATE INDEX IF NOT EXISTS idx_license_processing_stages_workflow_id ON license_processing_stages(workflow_id);

CREATE INDEX IF NOT EXISTS idx_system_config_settings_group_id ON system_config_settings(group_id);
CREATE INDEX IF NOT EXISTS idx_system_config_settings_key ON system_config_settings(key);

CREATE INDEX IF NOT EXISTS idx_email_logs_recipient ON email_logs(recipient_email);
CREATE INDEX IF NOT EXISTS idx_email_logs_status ON email_logs(status);
CREATE INDEX IF NOT EXISTS idx_email_logs_sent_at ON email_logs(sent_at);

-- Insert default system configuration groups
INSERT INTO system_config_groups (name, display_name, description, sort_order) VALUES
('general', 'General Settings', 'General platform configuration', 1),
('email', 'Email Configuration', 'SMTP and email settings', 2),
('license', 'License Settings', 'License processing configuration', 3),
('payment', 'Payment Settings', 'Payment gateway configuration', 4),
('security', 'Security Settings', 'Authentication and security rules', 5),
('notification', 'Notification Settings', 'Notification preferences and rules', 6)
ON CONFLICT (name) DO NOTHING;

-- Insert default email templates
INSERT INTO email_templates (name, subject, html_body, text_body, variables) VALUES
('welcome', 'Welcome to SaaS UMKM Platform!', 
 '<h1>Welcome {{user_name}}!</h1><p>Thank you for joining our platform.</p>', 
 'Welcome {{user_name}}! Thank you for joining our platform.',
 '["user_name", "company_name"]'),
('email_verification', 'Verify Your Email Address',
 '<h1>Email Verification</h1><p>Hi {{user_name}}, please verify your email by clicking the link below:</p><a href="{{verification_link}}">Verify Email</a>',
 'Hi {{user_name}}, please verify your email: {{verification_link}}',
 '["user_name", "verification_link"]'),
('license_approved', 'License Application Approved',
 '<h1>Congratulations!</h1><p>Your license application for {{company_name}} has been approved.</p>',
 'Congratulations! Your license application for {{company_name}} has been approved.',
 '["user_name", "company_name", "license_type", "license_number"]')
ON CONFLICT (name) DO NOTHING;
