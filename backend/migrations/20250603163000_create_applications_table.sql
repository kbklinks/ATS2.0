-- application_stage enum is now created in a separate migration (20250603162500_create_application_stage_enum.sql)

CREATE TABLE IF NOT EXISTS applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id INTEGER NOT NULL, -- Change to UUID if jobs table uses UUID
    candidate_id UUID NOT NULL,
    stage application_stage NOT NULL DEFAULT 'applied',
    referred_by UUID,
    application_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resume_url TEXT,
    cover_letter_url TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (candidate_id) REFERENCES users(id),
    FOREIGN KEY (referred_by) REFERENCES users(id)
);

-- Down Migration
DROP TABLE IF EXISTS applications;
-- application_stage enum is dropped in its own migration
