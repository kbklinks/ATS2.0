# Deployment to Google Cloud Run

## Backend

1. Build Docker image:
   ```bash
   docker build -t gcr.io/YOUR_PROJECT_ID/ats-backend ./backend
   ```
2. Push to Google Container Registry:
   ```bash
   docker push gcr.io/YOUR_PROJECT_ID/ats-backend
   ```
3. Deploy to Cloud Run:
   ```bash
   gcloud run deploy ats-backend --image gcr.io/YOUR_PROJECT_ID/ats-backend --platform managed --region YOUR_REGION --allow-unauthenticated
   ```

## Frontend

1. Build Docker image:
   ```bash
   docker build -t gcr.io/YOUR_PROJECT_ID/ats-frontend ./frontend
   ```
2. Push to Google Container Registry:
   ```bash
   docker push gcr.io/YOUR_PROJECT_ID/ats-frontend
   ```
3. Deploy to Cloud Run:
   ```bash
   gcloud run deploy ats-frontend --image gcr.io/YOUR_PROJECT_ID/ats-frontend --platform managed --region YOUR_REGION --allow-unauthenticated
   ```

---

- Replace `YOUR_PROJECT_ID` and `YOUR_REGION` as needed.
- Ensure environment variables are set securely in Cloud Run.
