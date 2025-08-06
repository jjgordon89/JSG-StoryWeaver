import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ProjectSummary {
  project: {
    id: string;
    name: string;
    description: string | null;
    genre: string | null;
    target_word_count: number | null;
    current_word_count: number;
    status: string;
    created_at: string;
    updated_at: string;
  };
  document_count: number;
  character_count: number;
  location_count: number;
  recent_documents: Array<{
    id: string;
    title: string;
    document_type: string;
    word_count: number;
    updated_at: string;
  }>;
  recent_activity: Array<{
    activity_type: string;
    description: string;
    timestamp: string;
    related_id?: string;
  }>;
  word_count_history: Array<{
    date: string;
    count: number;
  }>;
}

interface ProjectPreviewProps {
  projectId: string;
  onClose: () => void;
  onOpen: () => void;
}

const ProjectPreview: React.FC<ProjectPreviewProps> = ({ projectId, onClose, onOpen }) => {
  const [summary, setSummary] = useState<ProjectSummary | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchProjectSummary = async () => {
      setLoading(true);
      try {
        const result = await invoke<{ success: boolean; data: ProjectSummary; error?: string }>(
          'get_project_preview', 
          { project_id: projectId }
        );
        
        if (result.success && result.data) {
          setSummary(result.data);
        } else {
          setError(result.error || 'Failed to load project summary');
        }
      } catch (err) {
        setError(`Error: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    fetchProjectSummary();
  }, [projectId]);

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-xl w-full max-w-2xl">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-xl font-bold">Project Preview</h2>
            <button 
              onClick={onClose}
              className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div className="flex items-center justify-center h-64">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-xl w-full max-w-2xl">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-xl font-bold">Error</h2>
            <button 
              onClick={onClose}
              className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div className="text-red-500 dark:text-red-400">{error}</div>
        </div>
      </div>
    );
  }

  if (!summary) {
    return null;
  }

  const { project, document_count, character_count, location_count, recent_activity, recent_documents, word_count_history } = summary;

  // Calculate completion percentage if target word count exists
  const completionPercentage = project.target_word_count 
    ? Math.round((project.current_word_count / project.target_word_count) * 100) 
    : null;

  // Format dates
  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-xl w-full max-w-2xl">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold">{project.name}</h2>
          <button 
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Project Details */}
          <div className="space-y-4">
            <div>
              <h3 className="text-lg font-semibold mb-2">Details</h3>
              <div className="space-y-2">
                {project.description && (
                  <p className="text-gray-600 dark:text-gray-300">{project.description}</p>
                )}
                <div className="flex items-center">
                  <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Status:</span>
                  <span className="capitalize">{project.status.toLowerCase()}</span>
                </div>
                {project.genre && (
                  <div className="flex items-center">
                    <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Genre:</span>
                    <span>{project.genre}</span>
                  </div>
                )}
                <div className="flex items-center">
                  <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Created:</span>
                  <span>{formatDate(project.created_at)}</span>
                </div>
                <div className="flex items-center">
                  <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Last updated:</span>
                  <span>{formatDate(project.updated_at)}</span>
                </div>
              </div>
            </div>

            {/* Word Count Progress */}
            <div>
              <h3 className="text-lg font-semibold mb-2">Progress</h3>
              <div className="space-y-2">
                <div className="flex items-center">
                  <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Word count:</span>
                  <span>{project.current_word_count.toLocaleString()}</span>
                </div>
                {project.target_word_count && (
                  <>
                    <div className="flex items-center">
                      <span className="text-sm font-medium text-gray-500 dark:text-gray-400 w-24">Target:</span>
                      <span>{project.target_word_count.toLocaleString()}</span>
                    </div>
                    <div>
                      <div className="flex justify-between mb-1">
                        <span className="text-xs font-medium text-blue-600 dark:text-blue-400">
                          {completionPercentage}% Complete
                        </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                        <div 
                          className="bg-blue-600 h-2.5 rounded-full" 
                          style={{ width: `${Math.min(completionPercentage || 0, 100)}%` }}
                        ></div>
                      </div>
                    </div>
                  </>
                )}
              </div>
            </div>
          </div>

          {/* Project Stats */}
          <div className="space-y-4">
            <div>
              <h3 className="text-lg font-semibold mb-2">Statistics</h3>
              <div className="grid grid-cols-3 gap-4">
                <div className="bg-blue-50 dark:bg-blue-900/30 p-3 rounded-lg text-center">
                  <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">{document_count}</div>
                  <div className="text-sm text-gray-600 dark:text-gray-400">Documents</div>
                </div>
                <div className="bg-green-50 dark:bg-green-900/30 p-3 rounded-lg text-center">
                  <div className="text-2xl font-bold text-green-600 dark:text-green-400">{character_count}</div>
                  <div className="text-sm text-gray-600 dark:text-gray-400">Characters</div>
                </div>
                <div className="bg-purple-50 dark:bg-purple-900/30 p-3 rounded-lg text-center">
                  <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">{location_count}</div>
                  <div className="text-sm text-gray-600 dark:text-gray-400">Locations</div>
                </div>
              </div>
            </div>

            {/* Recent Documents */}
            <div>
              <h3 className="text-lg font-semibold mb-2">Recent Documents</h3>
              {recent_documents && recent_documents.length > 0 ? (
                <ul className="space-y-2 max-h-32 overflow-y-auto">
                  {recent_documents.map((doc) => (
                    <li key={doc.id} className="text-sm p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
                      <div className="flex justify-between items-center">
                        <div className="flex items-center">
                          <span className="text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-0.5 rounded mr-2">
                            {doc.document_type}
                          </span>
                          <span className="truncate">{doc.title}</span>
                        </div>
                        <span className="text-xs text-gray-500 dark:text-gray-400">
                          {doc.word_count} words
                        </span>
                      </div>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-gray-500 dark:text-gray-400">No documents yet</p>
              )}
            </div>

            {/* Recent Activity */}
            <div>
              <h3 className="text-lg font-semibold mb-2">Recent Activity</h3>
              {recent_activity.length > 0 ? (
                <ul className="space-y-2 max-h-32 overflow-y-auto">
                  {recent_activity.map((activity, index) => (
                    <li key={index} className="text-sm">
                      <div className="flex items-start">
                        <div className="w-2 h-2 mt-1.5 rounded-full bg-blue-500 mr-2"></div>
                        <div>
                          <p>{activity.description}</p>
                          <p className="text-xs text-gray-500 dark:text-gray-400">
                            {new Date(activity.timestamp).toLocaleString()}
                          </p>
                        </div>
                      </div>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-gray-500 dark:text-gray-400">No recent activity</p>
              )}
            </div>

            {/* Word Count History */}
            {word_count_history && word_count_history.length > 0 && (
              <div>
                <h3 className="text-lg font-semibold mb-2">Progress</h3>
                <div className="h-32 relative">
                  <div className="absolute inset-0 flex items-end">
                    {word_count_history.map((item, index) => {
                      const maxCount = Math.max(...word_count_history.map(h => h.count));
                      const height = maxCount > 0 ? (item.count / maxCount) * 100 : 0;
                      return (
                        <div 
                          key={index}
                          className="flex-1 mx-0.5 bg-blue-500 dark:bg-blue-600 rounded-t"
                          style={{ height: `${height}%` }}
                          title={`${item.date}: ${item.count} words`}
                        />
                      );
                    })}
                  </div>
                </div>
                <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-1">
                  <span>{word_count_history[0].date}</span>
                  <span>{word_count_history[word_count_history.length - 1].date}</span>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="mt-6 flex justify-end space-x-3">
          <button
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            Close
          </button>
          <button
            onClick={onOpen}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Open Project
          </button>
        </div>
      </div>
    </div>
  );
};

export default ProjectPreview;
