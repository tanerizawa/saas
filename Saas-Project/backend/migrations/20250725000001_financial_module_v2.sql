-- Financial module tables for Phase 6

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    account_type TEXT NOT NULL,
    balance NUMERIC(15, 2) NOT NULL DEFAULT 0.00,
    currency TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status TEXT NOT NULL DEFAULT 'active'
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    amount NUMERIC(15, 2) NOT NULL,
    transaction_type TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status TEXT NOT NULL DEFAULT 'pending'
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_transactions_user_id ON transactions(user_id);
CREATE INDEX IF NOT EXISTS idx_transactions_account_id ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_created_at ON transactions(created_at);
CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);
CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);

-- Add triggers to update account balance when a transaction is created
CREATE OR REPLACE FUNCTION update_account_balance()
RETURNS TRIGGER AS $$
DECLARE
    balance_change NUMERIC;
BEGIN
    -- Calculate balance change based on transaction type
    IF NEW.transaction_type = 'deposit' THEN
        balance_change := NEW.amount;
    ELSIF NEW.transaction_type = 'withdrawal' THEN
        balance_change := -NEW.amount;
    ELSE
        balance_change := 0; -- For transfer or other types, handled separately
    END IF;
    
    -- Update the account balance
    UPDATE accounts 
    SET balance = balance + balance_change, 
        updated_at = NOW()
    WHERE id = NEW.account_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to update account balance when a transaction is created
CREATE TRIGGER update_account_balance_on_transaction
AFTER INSERT ON transactions
FOR EACH ROW
WHEN (NEW.status = 'completed')
EXECUTE FUNCTION update_account_balance();

-- Add comments for documentation
COMMENT ON TABLE accounts IS 'Financial accounts owned by users';
COMMENT ON TABLE transactions IS 'Financial transactions linked to accounts';
COMMENT ON COLUMN accounts.account_type IS 'Type of account: checking, savings, investment, credit, cash';
COMMENT ON COLUMN transactions.transaction_type IS 'Type of transaction: deposit, withdrawal, transfer';
