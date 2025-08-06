import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Document {
  id: string;
  title: string;
  document_type: string;
  project_id: string;
}

interface DocumentLink {
  id: string;
  from_document_id: string;
  to_document_id: string;
  link_order: number;
  created_at: string;
}

interface DocumentLinkingProps {
  documentId: string;
  projectId: string;
  onDocumentSelect?: (documentId: string) => void;
}

const DocumentLinking: React.FC<DocumentLinkingProps> = ({ documentId, projectId, onDocumentSelect }) => {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [links, setLinks] = useState<DocumentLink[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [previousDocuments, setPreviousDocuments] = useState<Document[]>([]);
  const [nextDocuments, setNextDocuments] = useState<Document[]>([]);
  const [availableDocuments, setAvailableDocuments] = useState<Document[]>([]);
  const [selectedLinkType, setSelectedLinkType] = useState<'previous' | 'next'>('next');
  const [selectedDocumentId, setSelectedDocumentId] = useState<string | null>(null);

  // Fetch documents and links
  useEffect(() => {
    const fetchData = async () => {
      setLoading(true);
      try {
        // Fetch all documents in the project
        const docsResponse = await invoke<{ data: Document[] }>('get_documents', { projectId });
        if (!docsResponse.data) {
          throw new Error('Failed to fetch documents');
        }
        setDocuments(docsResponse.data);
        
        // Fetch document links for the current document
        const linksResponse = await invoke<{ data: DocumentLink[] }>('get_all_links_for_document', { documentId });
        if (!linksResponse.data) {
          throw new Error('Failed to fetch document links');
        }
        setLinks(linksResponse.data);
      } catch (err) {
        console.error('Error fetching document links:', err);
        setError('Failed to load document links');
      } finally {
        setLoading(false);
      }
    };
    
    fetchData();
  }, [documentId, projectId]);

  // Process links to determine previous and next documents
  useEffect(() => {
    if (!documents.length || !links.length) return;
    
    // Find previous documents (documents that link to this one)
    const prev = links
      .filter(link => link.to_document_id === documentId)
      .map(link => documents.find(doc => doc.id === link.from_document_id))
      .filter((doc): doc is Document => doc !== undefined)
      .sort((a, b) => {
        const linkA = links.find(link => link.from_document_id === a.id && link.to_document_id === documentId);
        const linkB = links.find(link => link.from_document_id === b.id && link.to_document_id === documentId);
        return (linkA?.link_order || 0) - (linkB?.link_order || 0);
      });
    
    // Find next documents (documents that this one links to)
    const next = links
      .filter(link => link.from_document_id === documentId)
      .map(link => documents.find(doc => doc.id === link.to_document_id))
      .filter((doc): doc is Document => doc !== undefined)
      .sort((a, b) => {
        const linkA = links.find(link => link.to_document_id === a.id && link.from_document_id === documentId);
        const linkB = links.find(link => link.to_document_id === b.id && link.from_document_id === documentId);
        return (linkA?.link_order || 0) - (linkB?.link_order || 0);
      });
    
    // Find available documents (documents that could be linked but aren't yet)
    const linkedDocIds = new Set([
      ...prev.map(doc => doc.id),
      ...next.map(doc => doc.id),
      documentId // Exclude the current document
    ]);
    
    const available = documents
      .filter(doc => !linkedDocIds.has(doc.id) && doc.document_type === 'chapter')
      .sort((a, b) => a.title.localeCompare(b.title));
    
    setPreviousDocuments(prev);
    setNextDocuments(next);
    setAvailableDocuments(available);
  }, [documents, links, documentId]);

  // Create a new link
  const handleCreateLink = async () => {
    if (!selectedDocumentId) return;
    
    try {
      // Create request based on link type
      const request = {
        from_document_id: selectedLinkType === 'previous' ? selectedDocumentId : documentId,
        to_document_id: selectedLinkType === 'previous' ? documentId : selectedDocumentId,
        link_order: 1 // Default order
      };
      
      // Call the backend API to create the link
      const response = await invoke<{ data: DocumentLink }>('create_document_link', { request });
      
      if (!response.data) {
        throw new Error('Failed to create document link');
      }
      
      // Update links state with the new link
      setLinks([...links, response.data]);
      setSelectedDocumentId(null);
    } catch (err) {
      console.error('Error creating document link:', err);
      setError('Failed to create document link');
    }
  };

  // Remove a link
  const handleRemoveLink = async (linkId: string) => {
    try {
      // Call the backend API to delete the link
      await invoke('delete_document_link', { id: linkId });
      
      // Update links state
      setLinks(links.filter(link => link.id !== linkId));
    } catch (err) {
      console.error('Error removing document link:', err);
      setError('Failed to remove document link');
    }
  };

  if (loading) {
    return <div className="p-4">Loading document links...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">{error}</div>;
  }

  return (
    <div className="document-linking p-4 bg-white dark:bg-gray-800 rounded-md shadow">
      <h3 className="text-lg font-semibold mb-4">Document Continuity</h3>
      
      {/* Previous Documents */}
      <div className="mb-4">
        <h4 className="font-medium text-sm text-gray-500 dark:text-gray-400 mb-2">Previous Documents</h4>
        {previousDocuments.length === 0 ? (
          <p className="text-sm text-gray-500">No previous documents linked.</p>
        ) : (
          <ul className="space-y-1">
            {previousDocuments.map(doc => {
              const link = links.find(l => l.from_document_id === doc.id && l.to_document_id === documentId);
              return (
                <li key={doc.id} className="flex justify-between items-center p-2 bg-gray-50 dark:bg-gray-700 rounded">
                  <div 
                    className="flex-grow cursor-pointer hover:underline"
                    onClick={() => onDocumentSelect && onDocumentSelect(doc.id)}
                  >
                    <span className="mr-2">◀️</span>
                    {doc.title}
                  </div>
                  {link && (
                    <button
                      className="text-xs text-red-500 hover:text-red-700"
                      onClick={() => handleRemoveLink(link.id)}
                    >
                      Remove
                    </button>
                  )}
                </li>
              );
            })}
          </ul>
        )}
      </div>
      
      {/* Next Documents */}
      <div className="mb-4">
        <h4 className="font-medium text-sm text-gray-500 dark:text-gray-400 mb-2">Next Documents</h4>
        {nextDocuments.length === 0 ? (
          <p className="text-sm text-gray-500">No next documents linked.</p>
        ) : (
          <ul className="space-y-1">
            {nextDocuments.map(doc => {
              const link = links.find(l => l.from_document_id === documentId && l.to_document_id === doc.id);
              return (
                <li key={doc.id} className="flex justify-between items-center p-2 bg-gray-50 dark:bg-gray-700 rounded">
                  <div 
                    className="flex-grow cursor-pointer hover:underline"
                    onClick={() => onDocumentSelect && onDocumentSelect(doc.id)}
                  >
                    <span className="mr-2">▶️</span>
                    {doc.title}
                  </div>
                  {link && (
                    <button
                      className="text-xs text-red-500 hover:text-red-700"
                      onClick={() => handleRemoveLink(link.id)}
                    >
                      Remove
                    </button>
                  )}
                </li>
              );
            })}
          </ul>
        )}
      </div>
      
      {/* Add New Link */}
      {availableDocuments.length > 0 && (
        <div className="mt-6 p-3 border rounded-md bg-gray-50 dark:bg-gray-700">
          <h4 className="text-sm font-medium mb-2">Add New Link</h4>
          
          <div className="flex space-x-2 mb-2">
            <button
              className={`flex-1 py-1 px-2 rounded text-sm ${
                selectedLinkType === 'previous' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300'
              }`}
              onClick={() => setSelectedLinkType('previous')}
            >
              Previous Document
            </button>
            <button
              className={`flex-1 py-1 px-2 rounded text-sm ${
                selectedLinkType === 'next' 
                  ? 'bg-blue-500 text-white' 
                  : 'bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300'
              }`}
              onClick={() => setSelectedLinkType('next')}
            >
              Next Document
            </button>
          </div>
          
          <select
            className="w-full p-2 mb-2 border rounded dark:bg-gray-600 dark:text-white"
            value={selectedDocumentId || ''}
            onChange={(e) => setSelectedDocumentId(e.target.value || null)}
          >
            <option value="">Select a document...</option>
            {availableDocuments.map(doc => (
              <option key={doc.id} value={doc.id}>
                {doc.title}
              </option>
            ))}
          </select>
          
          <button
            className="w-full bg-green-500 hover:bg-green-700 text-white py-1 px-2 rounded disabled:opacity-50"
            onClick={handleCreateLink}
            disabled={!selectedDocumentId}
          >
            Create Link
          </button>
        </div>
      )}
    </div>
  );
};

export default DocumentLinking;
