CREATE TABLE IF NOT EXISTS x_snipe_targets (
    id SERIAL PRIMARY KEY,
    target_name TEXT NOT NULL,
    sol_amount FLOAT NOT NULL DEFAULT 1.0,
    slippage INTEGER NOT NULL DEFAULT 15,
    priority_fee FLOAT NOT NULL DEFAULT 0.1,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deactivate_on_snipe BOOLEAN NOT NULL DEFAULT TRUE
);
