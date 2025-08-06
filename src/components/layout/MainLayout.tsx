import React from 'react';
import ProjectView from '../../features/projects/ProjectView';

const MainLayout: React.FC = () => {
  return (
    <div className="flex h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      {/* Left Column (Navigation) */}
      <div className="w-1/4 bg-gray-200 dark:bg-gray-800 p-4">
        <ProjectView />
      </div>

      {/* Middle Column (Editor) */}
      <div className="flex-1 p-4">
        <h2 className="text-xl font-bold mb-4">Document Editor</h2>
        {/* Placeholder for Monaco Editor */}
        <div className="w-full h-full bg-white dark:bg-gray-700 rounded-md p-2">
          Editor Area
        </div>
      </div>

      {/* Right Column (History/Cards) */}
      <div className="w-1/4 bg-gray-200 dark:bg-gray-800 p-4">
        <h2 className="text-xl font-bold mb-4">History & Cards</h2>
        {/* Placeholder for AI cards */}
        <p>AI Cards Panel</p>
      </div>
    </div>
  );
};

export default MainLayout;
