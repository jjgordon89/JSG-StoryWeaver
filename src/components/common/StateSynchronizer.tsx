import React, { useEffect } from 'react';
import { useStateSynchronization } from '../../utils/stateSynchronizer';

/**
 * StateSynchronizer component
 * 
 * This component initializes the state synchronization system when mounted.
 * It should be included near the root of the application to ensure state
 * synchronization is active throughout the app lifecycle.
 */
const StateSynchronizer: React.FC = () => {
  // Initialize state synchronization
  useStateSynchronization();
  
  return null; // This component doesn't render anything
};

export default StateSynchronizer;
