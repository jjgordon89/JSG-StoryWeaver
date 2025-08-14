import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Canvas, CanvasCollaborationSession } from '../../types/canvas';
import './CanvasCollaboration.css';

interface CanvasCollaborationProps {
  canvas: Canvas;
  onClose: () => void;
}

export const CanvasCollaboration: React.FC<CanvasCollaborationProps> = ({
  canvas,
  onClose
}) => {
  const [session, setSession] = useState<CanvasCollaborationSession | null>(null);
  const [userName, setUserName] = useState('');
  const [sessionToken, setSessionToken] = useState('');
  const [isJoining, setIsJoining] = useState(false);
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [participants, setParticipants] = useState<string[]>([]);

  useEffect(() => {
    checkExistingSession();
  }, [canvas.id]);

  const checkExistingSession = async () => {
    try {
      // Check if there's already an active session for this canvas
      const existingSession = await invoke<CanvasCollaborationSession | null>('get_canvas_collaboration_session_by_canvas_id', {
        canvasId: canvas.id
      });
      
      if (existingSession) {
        setSession(existingSession);
        setParticipants(JSON.parse(existingSession.participants || '[]'));
      }
    } catch (err) {
      // No existing session, which is fine
    }
  };

  const createSession = async () => {
    if (!userName.trim()) {
      setError('Please enter your name');
      return;
    }

    try {
      setIsCreating(true);
      setError(null);

      const newSessionToken = await invoke<string>('join_canvas_collaboration', {
        canvasId: canvas.id,
        userName: userName.trim()
      });

      setSessionToken(newSessionToken);
      
      // Get the session details
      const sessionData = await invoke<CanvasCollaborationSession>('get_canvas_collaboration_session', {
        sessionToken: newSessionToken
      });

      if (sessionData) {
        setSession(sessionData);
        setParticipants(JSON.parse(sessionData.participants || '[]'));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create collaboration session');
    } finally {
      setIsCreating(false);
    }
  };

  const joinSession = async () => {
    if (!userName.trim() || !sessionToken.trim()) {
      setError('Please enter your name and session token');
      return;
    }

    try {
      setIsJoining(true);
      setError(null);

      await invoke('join_canvas_collaboration_session', {
        sessionToken: sessionToken.trim()
      });

      // Get the session details
      const sessionData = await invoke<CanvasCollaborationSession>('get_canvas_collaboration_session', {
        sessionToken: sessionToken.trim()
      });

      if (sessionData) {
        setSession(sessionData);
        setParticipants(JSON.parse(sessionData.participants || '[]'));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to join collaboration session');
    } finally {
      setIsJoining(false);
    }
  };

  const leaveSession = async () => {
    if (!session || !userName) return;

    try {
      await invoke('leave_canvas_collaboration', {
        sessionToken: session.session_token,
        userName: userName
      });

      setSession(null);
      setParticipants([]);
      setSessionToken('');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to leave session');
    }
  };

  const copySessionToken = () => {
    if (session) {
      navigator.clipboard.writeText(session.session_token);
      // You could add a toast notification here
    }
  };

  const formatExpiresAt = (expiresAt?: string) => {
    if (!expiresAt) return 'Never';
    const date = new Date(expiresAt);
    return date.toLocaleString();
  };

  return (
    <div className="canvas-collaboration">
      <div className="collaboration-overlay" onClick={onClose}></div>
      <div className="collaboration-panel">
        <div className="collaboration-header">
          <h3>Canvas Collaboration</h3>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <div className="collaboration-content">
          {error && (
            <div className="error-message">
              {error}
              <button onClick={() => setError(null)}>×</button>
            </div>
          )}

          {!session ? (
            <div className="no-session">
              <div className="form-group">
                <label htmlFor="user-name">Your Name:</label>
                <input
                  id="user-name"
                  type="text"
                  value={userName}
                  onChange={(e) => setUserName(e.target.value)}
                  className="form-control"
                  placeholder="Enter your name..."
                />
              </div>

              <div className="session-actions">
                <div className="create-session">
                  <h4>Start New Session</h4>
                  <p>Create a new collaboration session for this canvas.</p>
                  <button
                    className="btn btn-primary"
                    onClick={createSession}
                    disabled={isCreating || !userName.trim()}
                  >
                    {isCreating ? 'Creating...' : 'Start Session'}
                  </button>
                </div>

                <div className="join-session">
                  <h4>Join Existing Session</h4>
                  <div className="form-group">
                    <label htmlFor="session-token">Session Token:</label>
                    <input
                      id="session-token"
                      type="text"
                      value={sessionToken}
                      onChange={(e) => setSessionToken(e.target.value)}
                      className="form-control"
                      placeholder="Enter session token..."
                    />
                  </div>
                  <button
                    className="btn btn-secondary"
                    onClick={joinSession}
                    disabled={isJoining || !userName.trim() || !sessionToken.trim()}
                  >
                    {isJoining ? 'Joining...' : 'Join Session'}
                  </button>
                </div>
              </div>
            </div>
          ) : (
            <div className="active-session">
              <div className="session-info">
                <h4>Active Collaboration Session</h4>
                <div className="session-details">
                  <div className="detail-item">
                    <label>Session Token:</label>
                    <div className="token-display">
                      <code>{session.session_token}</code>
                      <button
                        className="copy-btn"
                        onClick={copySessionToken}
                        title="Copy to clipboard"
                      >
                        📋
                      </button>
                    </div>
                  </div>
                  
                  <div className="detail-item">
                    <label>Participants:</label>
                    <span>{session.current_participants} / {session.max_participants}</span>
                  </div>
                  
                  <div className="detail-item">
                    <label>Expires:</label>
                    <span>{formatExpiresAt(session.expires_at)}</span>
                  </div>
                </div>
              </div>

              <div className="participants-list">
                <h5>Current Participants:</h5>
                <ul>
                  {participants.map((participant, index) => (
                    <li key={index} className="participant-item">
                      <span className="participant-name">{participant}</span>
                      {participant === userName && (
                        <span className="you-badge">You</span>
                      )}
                    </li>
                  ))}
                </ul>
              </div>

              <div className="session-actions">
                <button
                  className="btn btn-danger"
                  onClick={leaveSession}
                >
                  Leave Session
                </button>
              </div>

              <div className="collaboration-tips">
                <h5>Collaboration Tips:</h5>
                <ul>
                  <li>Changes are synchronized in real-time</li>
                  <li>Share the session token with others to invite them</li>
                  <li>All participants can edit elements simultaneously</li>
                  <li>Use different colors to distinguish your work</li>
                </ul>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
