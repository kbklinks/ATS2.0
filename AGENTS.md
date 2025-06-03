# AGENTS.md

Talent Track ATS – AI Agent & Automation Operations Manual
Version: 1.0
Last Updated: June 3, 2025, 06:07 PM EDT
Audit Trail ID: 6e7f3b8a-3d2c-4e9b-a5f7-9d0e8c7f2a4b

---

1. Purpose

This document is the canonical guide for all AI agents, bots, and automation tools interacting with the Talent Track ATS codebase. It ensures consistent, secure, and compliant operations for code generation, debugging, testing, and deployment, enabling seamless collaboration across agents (e.g., GitHub Copilot, ChatGPT Codex, Perplexity) and human developers. It supports onboarding, auditability, and alignment with project goals, including GDPR Art. 7 and CCPA 1798.100 compliance.

2. Project Overview

Talent Track ATS is a modern Applicant Tracking System designed to streamline job postings, candidate management, and recruitment workflows. It prioritizes user experience, security, and regulatory compliance.

Goals:
- Deliver a secure, compliant MVP with robust signup/login, job CRUD, and candidate portal.
- Achieve 85% user activation rate (signup to dashboard navigation).
- Ensure 100% GDPR/CCPA compliance audit completion.

User Flows:
- Signup: Users register via Signup.tsx, sending POST /api/auth/signup with { username, password } (extendable for email, consent).
- Login: Authenticate via Login.tsx, access dashboard (Dashboard.tsx).
- Job CRUD: Create, read, update, delete jobs via JobsCRUD.tsx (admin-only).
- Candidate Portal: Apply to jobs, track status via CandidatePortal.tsx.

KPIs:
- User activation rate (signup completion).
- Compliance audit completion.
- Job posting and application submission rates.

Current Status:
- Backend server running on 0.0.0.0:8080 after resolving compilation errors (serde support for Uuid, DateTime<Utc>, enum derives).
- /health endpoint returns 200 OK.
- "Signup failed" error during testing, possibly due to validation (e.g., missing consent), CORS, or API issues. Prior 404s for /api/auth/signup, /api/auth/login, /api/users likely resolved via router nesting fixes.
- Planned features: Google Auth integration, email and consent fields for signup.

3. Tech Stack

| Layer      | Technologies                        | Tools/Packages                |
|-----------|-------------------------------------|-------------------------------|
| Frontend  | React, TypeScript, Tailwind CSS      | Vite, @react-oauth/google (planned) |
| Backend   | Rust, Axum                           | sqlx, jsonwebtoken, bcrypt, openidconnect (planned) |
| Database  | PostgreSQL                           | sqlx                          |
| Auth      | JWT, OAuth 2.0 (planned)             | jsonwebtoken, Google Auth     |
| DevOps    | Cargo, npm                           | GitHub Actions, Postman       |
| Env       | .env                                 | VITE_API_BASE_URL=http://localhost:8080/api |

Frontend Port: 5173 (Vite, npm run dev)
Backend Port: 8080 (Axum, cargo run)
Repository: https://github.com/YOUR-ORG/talent-track-ats

4. Codebase Structure

Frontend (/frontend)
| File Path                | Description             | Purpose                       |
|-------------------------|-------------------------|-------------------------------|
| src/App.tsx             | Main React component    | Navigation, auth state, routing|
| src/pages/Signup.tsx    | Signup form             | User registration UI          |
| src/pages/Login.tsx     | Login form              | User authentication UI        |
| src/pages/JobsCRUD.tsx  | Job CRUD operations     | Admin job management          |
| src/pages/Dashboard.tsx | Recruiter dashboard     | Insights and summaries        |
| src/pages/CandidatePortal.tsx | Candidate features | Job applications, status tracking |
| src/pages/HealthCheck.tsx | System health indicator| Displays backend/API status   |
| src/api/auth.ts         | API client for auth     | Calls /api/auth/* endpoints   |
| package.json            | Frontend dependencies   | Build config                  |
| .env                    | Environment variables   | VITE_API_BASE_URL             |

Backend (/backend)
| File Path                | Description             | Purpose                       |
|-------------------------|-------------------------|-------------------------------|
| src/main.rs              | Axum server entry point | Router, state, CORS, startup  |
| src/routes/auth.rs       | Auth endpoints          | Handles /api/auth/*           |
| src/routes/users.rs      | User CRUD endpoints     | Handles /api/users/*          |
| src/routes/jobs.rs       | Job CRUD endpoints      | Handles /api/jobs/*           |
| src/routes/applications.rs | Application endpoints | Handles /api/applications/*   |
| src/routes/health.rs     | Health check endpoint   | Returns /health status        |
| src/models/user.rs       | User model              | DB mapping, serde, validation |
| src/models/application.rs| Application model       | DB mapping, serde, validation |
| src/auth_middleware.rs   | JWT middleware          | Route protection              |
| Cargo.toml               | Rust dependencies       | Build config                  |


Database
Schema: users table (planned extension for email, consent).
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT,
    consent BOOLEAN,
    role TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);
```
| Reference         | Description                        |
|-------------------|------------------------------------|
| /docs/COMPLIANCE.md | Compliance documentation         |
| /docs/prompts.md    | Prompt examples for agents        |
| /docs/SECURITY.md   | Security policies                 |
| /docs/api/openapi.yaml | OpenAPI 3.0 API documentation (if available) |
| /frontend/.env.example | Frontend environment example   |
| /backend/.env.example  | Backend environment example    |
| Postman: ATS Auth collection | API test collection      |
| /logs/agents/, /logs/audit/ | Audit and agent logs      |

5. Agent Roles and Permissions

| Agent Name      | Role                | Scope                | Allowed Actions                        | Restrictions                |
|-----------------|---------------------|----------------------|----------------------------------------|-----------------------------|
| GitHub Copilot  | Code Assistant      | /frontend, /backend, /docs | Suggest code, refactor, generate tests, update docs | No direct commits to main   |
| ChatGPT Codex   | Code Generator      | /frontend, /backend  | Generate code, debug, write tests      | Requires PR review          |
| Perplexity      | Workflow Guidance   | All                  | Provide guidance, suggest fixes, compliance notes | No code execution          |
| GitHub Actions  | CI/CD Bot           | All                  | Run tests, build, deploy, report status| Cannot modify source code   |
| AuditBot (planned) | Compliance Monitor| /backend/agents/audit| Log consent, audit compliance          | Read-only, no code changes  |
| Sentry (planned)| Error Logging       | All                  | Report and triage errors               | Read-only, no code changes  |

6. Agent Setup and Onboarding

6.1 Coding Agents (Copilot, Codex, Perplexity)
- GitHub Copilot:
  - Enable in VS Code/Codespaces with GitHub account.
  - Use detailed prompts referencing AGENTS.md, COMPLIANCE.md, or code comments.
  - Tag commits with [Copilot]: e.g., [Copilot] Fixed signup validation.
- ChatGPT Codex:
  - Feed this AGENTS.md and codebase details for context.
  - Generate code in small, focused chunks (e.g., one endpoint or component).
  - Include tests and comments for all code.
- Perplexity:
  - Access via web or CLI with API key.
  - Use for workflow guidance, compliance checks, or debugging suggestions.
- Best Practices:
  - Log all AI-generated code in commit messages (e.g., [Codex] Added Google Auth).
  - Run npm run lint (frontend) or cargo clippy (backend) before committing.
  - Document prompts in /docs/prompts.md for auditability.

6.2 Backend Agents (Jobs, API Bots)
- Location: /backend/agents/ or /backend/jobs/
- Run Locally:
  cd backend
  cargo run
- Environment:
  - Ensure .env includes VITE_API_BASE_URL=http://localhost:8080/api, API keys, and DB credentials.
  - Example .env:
    VITE_API_BASE_URL=http://localhost:8080/api
    DATABASE_URL=postgres://user:pass@localhost:5432/ats
    JWT_SECRET=your-secret

6.3 CI/CD and Monitoring Agents
- GitHub Actions:
  - Configured in .github/workflows/.
  - Runs npm test (frontend) and cargo test (backend) on PRs.
  - Deploys to staging/production on merge to main.
- Sentry (Planned):
  - Will report errors with anonymized data.
  - Configured in /backend/agents/sentry/.

7. Agent Operations

7.1 Frontend Tasks
- Signup/Login:
  - Forms in Signup.tsx and Login.tsx POST to /api/auth/signup and /api/auth/login with { username, password, consent }.
  - Validate fields client-side (e.g., password length ≥ 8).
  - Display API error messages (e.g., { "error": "Username taken" }).
- Example API call in auth.ts:
  export async function signup(data: { username: string; password: string; consent: boolean }) {
    const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/auth/signup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return response.json();
  }
- Compliance:
  - Add consent checkbox in Signup.tsx with privacy policy link.
  - Log consent status in PostgreSQL via backend.
- Google Auth (Planned):
  - Use @react-oauth/google in Signup.tsx for OAuth 2.0.
  - Send Google ID token to POST /api/auth/google.

7.2 Backend Tasks
- Auth Handlers:
  - Validate username, password, consent in auth.rs for /signup.
  - Hash passwords with bcrypt (replacing Argon2 for consistency).
  - Issue JWTs with jsonwebtoken including role and expiry.
- Example signup_handler:
  use axum::{http::StatusCode, Json};
  use serde::Deserialize;
  use sqlx::PgPool;
  #[derive(Deserialize)]
  struct SignupRequest {
      username: String,
      password: String,
      consent: bool,
  }
  async fn signup_handler(Json(payload): Json<SignupRequest>, State(pool): State<PgPool>) -> impl axum::response::IntoResponse {
      if !payload.consent {
          return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Consent required" })));
      }
      let hashed_password = bcrypt::hash(payload.password, 12).unwrap();
      let result = sqlx::query!(
          "INSERT INTO users (username, password_hash, consent, created_at) VALUES ($1, $2, $3, NOW())",
          payload.username, hashed_password, payload.consent
      )
      .execute(&*pool)
      .await;
      match result {
          Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "message": "User created" }))),
          Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))),
      }
  }
- User CRUD:
  - Enforce unique username in users.rs.
  - Never expose password_hash in API responses.
- Middleware:
  - Use auth_middleware.rs for JWT validation on protected routes.
  - Enable CORS for http://localhost:5173.
- Database:
  - Ensure users table supports email, consent for future Google Auth.
  - Log consent events in users or separate audit table.

7.3 Compliance and Privacy
- GDPR Art. 7:
  - Log explicit consent at signup in users.consent.
  - Provide audit endpoints (e.g., /api/audit/consent).
- CCPA 1798.100:
  - Store minimal PII (e.g., username, email).
  - Plan /api/user/export and /api/user/delete endpoints.
- Google Auth:
  - Update privacy policy to disclose Google OAuth usage.
  - Log consent for Google Auth users.

8. Debugging and Testing

8.1 Debugging Signup Issues
- Steps:
  - Open browser DevTools (F12) > Network tab, inspect POST /api/auth/signup for status (e.g., 400, 409) and response (e.g., { "error": "Missing consent" }).
  - Run cargo run and check backend logs for errors during signup.
  - Verify CORS headers allow http://localhost:5173.
  - Check .env for VITE_API_BASE_URL=http://localhost:8080/api.
- Common Causes:
  - Validation errors (e.g., duplicate username, missing consent).
  - CORS misconfiguration.
  - API base URL mismatch.
- Fix Example:
  Add CORS in main.rs:
  use axum::http::HeaderValue;
  let cors = axum::middleware::from_fn(|req, next| async move {
      let mut response = next(req).await.unwrap();
      response.headers_mut().insert("Access-Control-Allow-Origin", HeaderValue::from_static("http://localhost:5173"));
      Ok(response)
  });

8.2 Testing
- Frontend: Run npm test for unit tests in /frontend/tests/.
- Backend: Run cargo test for integration tests in /backend/tests/.
- Postman: Use ATS Auth collection to test /api/auth/* endpoints.
- Health Check: Ensure curl http://localhost:8080/health returns 200 OK.

8.3 Troubleshooting
| Symptom         | Cause                        | Fix/Debug Steps                |
|-----------------|-----------------------------|-------------------------------|
| "Signup failed" | Validation, CORS, missing fields | Check DevTools, logs, CORS    |
| 404 Not Found   | Router misconfiguration      | Verify auth.rs, main.rs paths  |
| 401 Unauthorized| Invalid JWT, middleware issue| Check auth_middleware.rs       |
| Compilation error| Missing dependencies, serde issues | Run cargo check, update Cargo.toml |
| Agent not responding | API key missing, port conflict | Check .env, restart agent    |

9. Planned Features
- Google Auth: Integrate @react-oauth/google (frontend) and openidconnect (backend) for OAuth 2.0 signup.
- Job CRUD: Enhance JobsCRUD.tsx and /api/jobs/* with admin UI and status updates.
- Compliance Endpoints: Add /api/user/export and /api/user/delete for GDPR/CCPA.
- Audit Logging: Implement AuditBot for consent and auth event tracking.
- Error Handling: Improve API error messages and logging.

10. Agent Collaboration Workflow
- Pull latest code from main branch.
- Announce changes in PR description or AGENTS.md (e.g., [Copilot] Added Google Auth).
- Make atomic commits with clear messages.
- Run tests (npm test, cargo test) and lint (npm run lint, cargo clippy).
- Submit PR with [AGENT] tag and request review.
- Update AGENTS.md with new conventions or compliance notes.
- Merge after at least one human or agent review.

11. Adding New Agents
- Fork template from /backend/agents/template/.
- Document setup, permissions, and compliance in AGENTS.md.
- Update agent table in Section 5.
- Submit PR with [AGENT][New] tag.
- Review by engineering and compliance leads.

12. Audit and Logging
- Log Location: /logs/agents/ or cloud log aggregation (planned).
- Format: Include timestamp, agent ID, action summary (e.g., [2025-06-03 18:07 EDT][Copilot] Generated signup handler).
- Audit: Weekly reports to compliance officer, stored in /logs/audit/.
- Example Commit:
  [Codex] Fixed CORS for /api/auth/signup (2025-06-03)

13. Contact and Escalation
| Name         | Role                  | Contact                    |
|--------------|-----------------------|----------------------------|
| Alice Brown  | Chief Product Officer | alice@talenttrack.example  |
| Bob Smith    | Lead Engineer         | bob@talenttrack.example    |
| Jane Doe     | Data Protection Officer| jane@talenttrack.example  |
- Urgent Issues: Tag @project-lead in GitHub or Slack #agents-support.
- Compliance Concerns: Contact DPO and update SECURITY.md.
- Architectural Changes: Propose in AGENTS.md and seek consensus.

14. References
- Compliance: /docs/COMPLIANCE.md
- Prompts: /docs/prompts.md
- Security: /docs/SECURITY.md
- Postman: ATS Auth collection
- Logs: /logs/agents/, /logs/audit/

This AGENTS.md ensures all AI agents and automation tools operate transparently, securely, and compliantly within the Talent Track ATS project, ready for enterprise-scale audits and collaboration.
