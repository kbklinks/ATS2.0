import { useEffect, useState } from 'react';
import { fetchJobs } from '../api';

interface Job {
  id: number;
  title: string;
  description: string;
  company: string;
  location: string;
  posted_at: string;
}

export default function JobList() {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    fetchJobs()
      .then(setJobs)
      .catch(e => setError(e.message))
      .finally(() => setLoading(false));
  }, []);
  
  if (loading) {
    return (
      <div className="p-6">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-1/4 mb-6"></div>
          <div className="space-y-4">
            {[1, 2, 3].map(i => (
              <div key={i} className="bg-white rounded-lg shadow p-6">
                <div className="h-6 bg-gray-200 rounded w-3/4 mb-2"></div>
                <div className="h-4 bg-gray-200 rounded w-1/2 mb-2"></div>
                <div className="h-4 bg-gray-200 rounded w-1/4"></div>
              </div>
            ))}
          </div>
        </div>
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="p-6">
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <div className="flex">
            <div className="ml-3">
              <h3 className="text-sm font-medium text-red-800">
                Error loading jobs
              </h3>
              <div className="mt-2 text-sm text-red-700">
                {error}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
  
  return (
    <div className="p-6">
      <div className="mb-6">
        <h2 className="text-2xl font-bold text-gray-900">Job Listings</h2>
        <p className="mt-1 text-sm text-gray-600">
          {jobs.length} open position{jobs.length !== 1 ? 's' : ''} available
        </p>
      </div>
      
      <div className="space-y-4">
        {jobs.map(job => (
          <div key={job.id} className="bg-white rounded-lg shadow hover:shadow-md transition-shadow p-6">
            <div className="flex justify-between items-start">
              <div className="flex-1">
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  {job.title}
                </h3>
                <p className="text-gray-600 mb-3">{job.description}</p>
                <div className="flex items-center text-sm text-gray-500">
                  <span className="font-medium">{job.company}</span>
                  <span className="mx-2">•</span>
                  <span>{job.location}</span>
                  <span className="mx-2">•</span>
                  <span>Posted {new Date(job.posted_at).toLocaleDateString()}</span>
                </div>
              </div>
              <button className="ml-4 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors">
                Apply Now
              </button>
            </div>
          </div>
        ))}
      </div>
      
      {jobs.length === 0 && (
        <div className="text-center py-12">
          <div className="text-gray-500">
            <p className="text-lg">No jobs available at this time.</p>
            <p className="text-sm mt-2">Please check back later for new opportunities.</p>
          </div>
        </div>
      )}
    </div>
  );
}
