import { useState } from 'react';

export default function JobForm({ onSubmit, initial, loading }: {
  onSubmit: (data: any) => void;
  initial?: any;
  loading?: boolean;
}) {
  const [title, setTitle] = useState(initial?.title || '');
  const [description, setDescription] = useState(initial?.description || '');
  const [company, setCompany] = useState(initial?.company || '');
  const [location, setLocation] = useState(initial?.location || '');

  return (
    <form
      onSubmit={e => {
        e.preventDefault();
        onSubmit({ title, description, company, location });
      }}
      className="space-y-4"
    >
      <div>
        <label className="block text-sm font-medium">Title</label>
        <input className="mt-1 block w-full border rounded p-2" value={title} onChange={e => setTitle(e.target.value)} required />
      </div>
      <div>
        <label className="block text-sm font-medium">Description</label>
        <textarea className="mt-1 block w-full border rounded p-2" value={description} onChange={e => setDescription(e.target.value)} required />
      </div>
      <div>
        <label className="block text-sm font-medium">Company</label>
        <input className="mt-1 block w-full border rounded p-2" value={company} onChange={e => setCompany(e.target.value)} required />
      </div>
      <div>
        <label className="block text-sm font-medium">Location</label>
        <input className="mt-1 block w-full border rounded p-2" value={location} onChange={e => setLocation(e.target.value)} required />
      </div>
      <button type="submit" className="w-full bg-blue-600 text-white py-2 rounded" disabled={loading}>
        {loading ? 'Saving...' : 'Save Job'}
      </button>
    </form>
  );
}
