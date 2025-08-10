# Tech Context: [Project Name]

## Technology Stack

### Frontend
- **Framework**: [React/Vue/Angular/etc.]
- **State Management**: [Redux/MobX/Context API/etc.]
- **Styling**: [CSS/SASS/Styled Components/Tailwind/etc.]
- **Build Tool**: [Webpack/Vite/Parcel/etc.]
- **Testing**: [Jest/Cypress/Testing Library/etc.]

### Backend
- **Language**: [Node.js/Python/Java/etc.]
- **Framework**: [Express/Django/Spring/etc.]
- **API Type**: [REST/GraphQL/gRPC/etc.]
- **Authentication**: [JWT/OAuth/Session/etc.]
- **Testing**: [Unit/Integration test frameworks]

### Database
- **Primary DB**: [PostgreSQL/MySQL/MongoDB/etc.]
- **Caching**: [Redis/Memcached/etc.]
- **ORM/ODM**: [Sequelize/TypeORM/Mongoose/etc.]
- **Migration Tool**: [What's used for schema management]

### Infrastructure
- **Hosting**: [AWS/Azure/GCP/Heroku/etc.]
- **Container**: [Docker/Kubernetes/etc.]
- **CI/CD**: [GitHub Actions/Jenkins/CircleCI/etc.]
- **Monitoring**: [DataDog/New Relic/CloudWatch/etc.]

## Development Setup

### Prerequisites
- **Runtime**: [Node.js version X, Python version Y, etc.]
- **Package Manager**: [npm/yarn/pip/poetry/etc.]
- **Database**: [Local setup requirements]
- **Other Tools**: [Docker, Redis, etc.]

### Installation Steps
1. Clone repository: `git clone [repo-url]`
2. Install dependencies: `[package-manager] install`
3. Set up environment: `cp .env.example .env`
4. Initialize database: `[migration command]`
5. Start development: `[dev command]`

### Environment Variables
```env
# Application
APP_ENV=development
APP_PORT=3000

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=project_db
DB_USER=username
DB_PASSWORD=password

# External Services
API_KEY=your-api-key
SMTP_HOST=smtp.example.com
```

## Development Workflow

### Branch Strategy
- **main/master**: Production-ready code
- **develop**: Integration branch
- **feature/***: New features
- **bugfix/***: Bug fixes
- **hotfix/***: Emergency fixes

### Code Standards
- **Linting**: [ESLint/Pylint/etc. configuration]
- **Formatting**: [Prettier/Black/etc. settings]
- **Pre-commit Hooks**: [Husky/pre-commit configuration]
- **Code Review**: [PR requirements]

### Testing Strategy
- **Unit Tests**: [Coverage requirements]
- **Integration Tests**: [Scope and tools]
- **E2E Tests**: [When and how]
- **Performance Tests**: [Benchmarks]

## Dependencies

### Core Dependencies
```json
{
  "dependency1": "^1.0.0",
  "dependency2": "^2.3.0"
}
```

### Dev Dependencies
```json
{
  "dev-dependency1": "^1.0.0",
  "dev-dependency2": "^2.3.0"
}
```

### Security Considerations
- **Dependency Scanning**: [Tool used]
- **Update Policy**: [How often/when to update]
- **Known Vulnerabilities**: [Tracking method]

## API Documentation

### Endpoints Overview
- **Auth**: `/api/auth/*`
- **Users**: `/api/users/*`
- **Resources**: `/api/resources/*`

### Authentication
- **Method**: [Bearer token/API key/etc.]
- **Header**: `Authorization: Bearer [token]`
- **Expiration**: [Token lifetime]

### Rate Limiting
- **Limits**: [Requests per minute/hour]
- **Headers**: [Rate limit headers returned]

## Deployment

### Build Process
1. Run tests: `[test command]`
2. Build assets: `[build command]`
3. Optimize: `[optimization steps]`
4. Package: `[packaging command]`

### Deployment Checklist
- [ ] All tests passing
- [ ] Environment variables configured
- [ ] Database migrations run
- [ ] SSL certificates valid
- [ ] Monitoring configured
- [ ] Backup strategy in place

### Rollback Procedure
1. [Step 1: How to identify issues]
2. [Step 2: How to rollback]
3. [Step 3: Verification steps]

## Troubleshooting

### Common Issues
1. **Issue**: [Description]
   - **Cause**: [Root cause]
   - **Solution**: [How to fix]

2. **Issue**: [Description]
   - **Cause**: [Root cause]
   - **Solution**: [How to fix]

### Debug Tools
- **Logging**: [Where logs are stored]
- **Debugging**: [Tools and techniques]
- **Profiling**: [Performance analysis tools]

### Support Contacts
- **Technical Lead**: [Contact info]
- **DevOps**: [Contact info]
- **External Support**: [Vendor contacts]
