import React from 'react';
import MainLayout from './components/layout/MainLayout';
import { ProjectProvider } from './contexts/ProjectContext';
import './App.css';

function App() {
  return (
    <ProjectProvider>
      <MainLayout />
    </ProjectProvider>
  );
}

export default App;
