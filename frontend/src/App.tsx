
import { useState } from 'react';
import JobsCRUD from './pages/JobsCRUD';
import './App.css';
import JobList from './pages/JobList';
import HealthCheck from './pages/HealthCheck';
import CandidatePortal from './pages/CandidatePortal';
import Login from './pages/Login';
import Signup from './pages/Signup';
import Dashboard from './pages/Dashboard';


function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [token, setToken] = useState<string | null>(null);
  const [showSignup, setShowSignup] = useState(false);

  if (!token) {
    return showSignup ? (
      <Signup onSignup={t => { setToken(t); setShowSignup(false); }} />
    ) : (
      <Login onLogin={setToken} onShowSignup={() => setShowSignup(true)} />
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <h1 className="text-3xl font-bold text-gray-900">Talent Track ATS</h1>
              <span className="ml-2 text-sm text-gray-500">v1.0</span>
            </div>
            <HealthCheck />
          </div>
        </div>
      </header>

      <nav className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex space-x-8">
            <button
              onClick={() => setActiveTab('dashboard')}
              className={`py-4 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'dashboard'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Dashboard
            </button>
            <button
              onClick={() => setActiveTab('jobs')}
              className={`py-4 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'jobs'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Job Listings
            </button>
            <button
              onClick={() => setActiveTab('candidates')}
              className={`py-4 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'candidates'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              Candidate Portal
            </button>
            <button
              onClick={() => { setToken(null); }}
              className="ml-auto py-2 px-4 bg-gray-200 rounded text-sm text-gray-700 hover:bg-gray-300"
            >
              Logout
            </button>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {activeTab === 'dashboard' && <Dashboard />}
        {activeTab === 'jobs' && token && <JobsCRUD token={token} />}
        {activeTab === 'candidates' && <CandidatePortal />}
      </main>
    </div>
  );
}

export default App
