import React from 'react';
import MainLayout from './components/layout/MainLayout';
import { ProjectProvider } from './contexts/ProjectContext';
import { ErrorProvider } from './components/providers/ErrorProvider';
import { ToastProvider } from './components/providers/ToastProvider';
import { ErrorBoundary } from './components/ui/ErrorBoundary';
import './App.css';

function App() {
  return (
    <ErrorBoundary>
      <ErrorProvider>
        <ToastProvider>
          <ProjectProvider>
            <MainLayout />
          </ProjectProvider>
        </ToastProvider>
      </ErrorProvider>
    </ErrorBoundary>
  );
}

export default App;
