import JobList from './JobList';

export default function Dashboard() {
  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold mb-4">Dashboard</h2>
      <JobList />
      {/* Add Users and Applications lists here as you expand */}
    </div>
  );
}
