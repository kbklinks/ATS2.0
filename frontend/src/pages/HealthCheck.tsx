import { useEffect, useState } from 'react';
import { fetchHealth } from '../api';

export default function HealthCheck() {
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    fetchHealth()
      .then(() => setError(null))
      .catch(e => setError(e.message))
      .finally(() => setLoading(false));
  }, []);
  
  if (loading) {
    return (
      <div className="flex items-center text-sm">
        <div className="w-2 h-2 bg-yellow-400 rounded-full mr-2"></div>
        Checking...
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="flex items-center text-sm text-red-600">
        <div className="w-2 h-2 bg-red-500 rounded-full mr-2"></div>
        Backend Offline
      </div>
    );
  }
  
  return (
    <div className="flex items-center text-sm text-green-600">
      <div className="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
      Backend Online
    </div>
  );
}
