-- Financial module tables for Phase 6
-- Add support for comprehensive financial management

-- Financial Accounts Table
CREATE TABLE IF NOT EXISTS financial_accounts (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    account_type JSONB NOT NULL,
    currency VARCHAR(3) NOT NULL,
    balance NUMERIC(19, 4) NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    description TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_financial_accounts_company_id ON financial_accounts(company_id);
CREATE INDEX IF NOT EXISTS idx_financial_accounts_name ON financial_accounts(name);
CREATE INDEX IF NOT EXISTS idx_financial_accounts_active ON financial_accounts(is_active);

-- Financial Transactions Table
CREATE TABLE IF NOT EXISTS financial_transactions (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    transaction_date TIMESTAMPTZ NOT NULL,
    transaction_type JSONB NOT NULL,
    amount NUMERIC(19, 4) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    description TEXT NOT NULL,
    reference_number VARCHAR(100),
    status JSONB NOT NULL,
    account_id UUID NOT NULL REFERENCES financial_accounts(id) ON DELETE RESTRICT,
    category_id UUID,
    tags JSONB DEFAULT '[]'::JSONB,
    attachments JSONB DEFAULT '[]'::JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID
);

CREATE INDEX IF NOT EXISTS idx_financial_transactions_company_id ON financial_transactions(company_id);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_account_id ON financial_transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_date ON financial_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_created_by ON financial_transactions(created_by);

-- Financial Categories Table
CREATE TABLE IF NOT EXISTS financial_categories (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    parent_id UUID REFERENCES financial_categories(id),
    is_expense BOOLEAN NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_financial_categories_company_id ON financial_categories(company_id);
CREATE INDEX IF NOT EXISTS idx_financial_categories_parent_id ON financial_categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_financial_categories_active ON financial_categories(is_active);

-- Add Foreign Key from financial_transactions to financial_categories
ALTER TABLE financial_transactions
ADD CONSTRAINT fk_financial_transactions_category
FOREIGN KEY (category_id) REFERENCES financial_categories(id);

-- Financial Budget Table
CREATE TABLE IF NOT EXISTS financial_budgets (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    amount NUMERIC(19, 4) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    category_id UUID REFERENCES financial_categories(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_financial_budgets_company_id ON financial_budgets(company_id);
CREATE INDEX IF NOT EXISTS idx_financial_budgets_category_id ON financial_budgets(category_id);
CREATE INDEX IF NOT EXISTS idx_financial_budgets_period ON financial_budgets(start_date, end_date);

-- Tax Settings Table
CREATE TABLE IF NOT EXISTS tax_settings (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    tax_name VARCHAR(100) NOT NULL,
    tax_rate NUMERIC(5, 2) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_tax_settings_company_id ON tax_settings(company_id);

-- Tax Reports Table
CREATE TABLE IF NOT EXISTS tax_reports (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    report_type VARCHAR(50) NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    total_income NUMERIC(19, 4) NOT NULL,
    total_expense NUMERIC(19, 4) NOT NULL,
    taxable_income NUMERIC(19, 4) NOT NULL,
    tax_amount NUMERIC(19, 4) NOT NULL,
    status VARCHAR(20) NOT NULL,
    submitted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID
);

CREATE INDEX IF NOT EXISTS idx_tax_reports_company_id ON tax_reports(company_id);
CREATE INDEX IF NOT EXISTS idx_tax_reports_period ON tax_reports(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_tax_reports_status ON tax_reports(status);
