import { useState } from 'react';
import { signup } from '../api/auth';

export default function Signup({ onSignup }: { onSignup: (token: string) => void }) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const data = await signup(username, password);
      if (!data.token) {
        setError(data.error || 'Signup failed: No token returned.');
        return;
      }
      onSignup(data.token);
    } catch (e: any) {
      // Try to parse backend error message if available
      if (e && e.response) {
        try {
          const errData = await e.response.json();
          setError(errData.error || errData.message || 'Signup failed.');
        } catch {
          setError(e.message || 'Signup failed.');
        }
      } else {
        setError(e.message || 'Signup failed.');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-md mx-auto mt-10 bg-white p-8 rounded shadow">
      <h2 className="text-2xl font-bold mb-6">Sign Up</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium">Username</label>
          <input type="text" className="mt-1 block w-full border rounded p-2" value={username} onChange={e => setUsername(e.target.value)} required />
        </div>
        <div>
          <label className="block text-sm font-medium">Password</label>
          <input type="password" className="mt-1 block w-full border rounded p-2" value={password} onChange={e => setPassword(e.target.value)} required />
        </div>
        {error && <div className="text-red-600 text-sm">{error}</div>}
        <button type="submit" className="w-full bg-blue-600 text-white py-2 rounded" disabled={loading}>{loading ? 'Signing up...' : 'Sign Up'}</button>
      </form>
    </div>
  );
}
