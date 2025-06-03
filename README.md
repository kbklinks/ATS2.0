# Talent Track ATS 1.0

An AI-powered, modular, enterprise-grade Applicant Tracking System built with modern technologies.

## Tech Stack

### Backend
- **Rust** with **Axum** web framework
- **SQLx** for database operations with **PostgreSQL**
- **Tokio** for async runtime
- **Serde** for JSON serialization
- **Tower-HTTP** for CORS middleware

### Frontend
- **React 19** with **TypeScript**
- **Vite** for build tooling and development server
- **Tailwind CSS** for styling
- **ESLint** for code quality

## Architecture

```
ATS2.0/
├── backend/          # Rust/Axum API server
│   ├── src/          # Rust source code
│   ├── migrations/   # Database migrations
│   └── tests/        # Backend tests
├── frontend/         # React/TypeScript UI
│   ├── src/          # React components and pages
│   └── public/       # Static assets
└── docs/            # Documentation
```

## Getting Started

### Prerequisites
- Rust 1.87+ (installed via rustup)
- Node.js 20+
- PostgreSQL (for production)

### Development Setup

1. **Start Backend Server**
   ```bash
   cd backend
   cargo run
   ```
   Backend runs on: http://localhost:8080

2. **Start Frontend Server**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   Frontend runs on: http://localhost:5174

### API Endpoints

- `GET /health` - Health check endpoint
- `GET /api/jobs` - List all job postings

### Features Implemented

✅ **Backend Infrastructure**
- Axum web server with async support
- CORS middleware for frontend communication
- JSON API endpoints
- Health check monitoring
- Sample job data API

✅ **Frontend Application**
- React component architecture
- TypeScript for type safety
- Responsive design with Tailwind CSS
- API integration with proxy configuration
- Job listings display
- Candidate portal interface
- Real-time backend status indicator

✅ **Development Environment**
- Hot reload for both frontend and backend
- VS Code tasks for build/run operations
- Environment configuration
- Error handling and loading states

### Next Steps

🔲 **Database Integration**
- PostgreSQL setup and connection
- Database migrations
- SQLx query implementation

🔲 **Authentication & Authorization**
- User authentication system
- Role-based access control
- JWT token management

🔲 **Advanced Features**
- File upload for resumes
- Advanced job search and filtering
- Application tracking workflow
- Email notifications
- Admin dashboard

## Development Commands

```bash
# Backend
cargo run          # Start development server
cargo test         # Run tests
cargo check        # Check compilation

# Frontend  
npm run dev        # Start development server
npm run build      # Production build
npm run lint       # Lint code
```

## Environment Variables

### Backend (.env)
```
DATABASE_URL=postgresql://user:password@localhost:5432/ats_db
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
```

### Frontend (.env.local)
```
VITE_API_BASE_URL=http://localhost:8080
VITE_APP_NAME=Talent Track ATS
VITE_APP_VERSION=1.0.0
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License.