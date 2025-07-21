-- Add migration script here

-- migrations/.../up.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE tanks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE water_quality_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tank_id UUID NOT NULL REFERENCES tanks(id) ON DELETE CASCADE,
    temperature FLOAT NOT NULL,     -- 水温（例：25.5°C）
    ph FLOAT NOT NULL,              -- pH（例：7.0）
    no2 FLOAT NOT NULL,             -- 亜硝酸塩（例：0.1 mg/L）
    no3 FLOAT NOT NULL,             -- 硝酸塩（例：20 mg/L）
    cl FLOAT NOT NULL,              -- 塩素（例：0.02 mg/L）
    gh FLOAT NOT NULL,              -- 総硬度（例：6 dGH）
    kh FLOAT NOT NULL,              -- 炭酸塩硬度（例：4 dKH）
    measured_at TIMESTAMP NOT NULL DEFAULT NOW()
);
