import { useEffect, useState } from 'react';
import { fetchJobs, createJob, updateJob, deleteJob } from '../api/jobs';
import JobForm from './JobForm';
import Toast from '../components/Toast';

export default function JobsCRUD({ token }: { token: string }) {
  const [jobs, setJobs] = useState<any[]>([]);
  const [editing, setEditing] = useState<any | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);
  const [loading, setLoading] = useState(false);

  const loadJobs = async () => {
    try {
      setLoading(true);
      setJobs(await fetchJobs(token));
    } catch (e: any) {
      setToast({ message: e.message, type: 'error' });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { loadJobs(); }, [token]);

  const handleSave = async (data: any) => {
    try {
      if (editing) {
        await updateJob(editing.id, data, token);
        setToast({ message: 'Job updated!', type: 'success' });
      } else {
        await createJob(data, token);
        setToast({ message: 'Job created!', type: 'success' });
      }
      setShowForm(false);
      setEditing(null);
      loadJobs();
    } catch (e: any) {
      setToast({ message: e.message, type: 'error' });
    }
  };

  const handleDelete = async (id: number) => {
    if (!window.confirm('Delete this job?')) return;
    try {
      await deleteJob(id, token);
      setToast({ message: 'Job deleted!', type: 'success' });
      loadJobs();
    } catch (e: any) {
      setToast({ message: e.message, type: 'error' });
    }
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Jobs</h2>
        <button className="bg-blue-600 text-white px-4 py-2 rounded" onClick={() => { setShowForm(true); setEditing(null); }}>Add Job</button>
      </div>
      {showForm && (
        <JobForm
          onSubmit={handleSave}
          initial={editing}
          loading={loading}
        />
      )}
      <div className="space-y-4 mt-4">
        {jobs.map(job => (
          <div key={job.id} className="bg-white rounded shadow p-4 flex justify-between items-center">
            <div>
              <div className="font-semibold">{job.title}</div>
              <div className="text-sm text-gray-500">{job.company} • {job.location}</div>
            </div>
            <div className="space-x-2">
              <button className="text-blue-600" onClick={() => { setEditing(job); setShowForm(true); }}>Edit</button>
              <button className="text-red-600" onClick={() => handleDelete(job.id)}>Delete</button>
            </div>
          </div>
        ))}
      </div>
      {toast && <Toast {...toast} onClose={() => setToast(null)} />}
    </div>
  );
}
