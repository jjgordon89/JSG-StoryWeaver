import React from 'react';
import { useSettingsStore } from '../../stores/settingsStore';

interface FocusModeSettingsProps {
  isOpen: boolean;
  onClose: () => void;
}

const FocusModeSettings: React.FC<FocusModeSettingsProps> = ({ isOpen, onClose }) => {
  const { focusModeOptions, updateFocusModeOptions } = useSettingsStore();

  if (!isOpen) return null;

  const handleToggleOption = (option: keyof typeof focusModeOptions) => {
    updateFocusModeOptions({ [option]: !focusModeOptions[option] });
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 w-96 max-w-full">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold">Focus Mode Settings</h2>
          <button 
            onClick={onClose}
            className="p-1 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M18 6L6 18M6 6l12 12"></path>
            </svg>
          </button>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <label htmlFor="hideNavigation" className="text-sm font-medium">
              Hide Navigation Panel
            </label>
            <div className="relative inline-block w-10 mr-2 align-middle select-none">
              <input 
                type="checkbox" 
                id="hideNavigation" 
                checked={focusModeOptions.hideNavigation}
                onChange={() => handleToggleOption('hideNavigation')}
                className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
              />
              <label 
                htmlFor="hideNavigation" 
                className={`toggle-label block overflow-hidden h-6 rounded-full cursor-pointer ${
                  focusModeOptions.hideNavigation ? 'bg-blue-500' : 'bg-gray-300'
                }`}
              ></label>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <label htmlFor="hideRightPanel" className="text-sm font-medium">
              Hide Right Panel
            </label>
            <div className="relative inline-block w-10 mr-2 align-middle select-none">
              <input 
                type="checkbox" 
                id="hideRightPanel" 
                checked={focusModeOptions.hideRightPanel}
                onChange={() => handleToggleOption('hideRightPanel')}
                className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
              />
              <label 
                htmlFor="hideRightPanel" 
                className={`toggle-label block overflow-hidden h-6 rounded-full cursor-pointer ${
                  focusModeOptions.hideRightPanel ? 'bg-blue-500' : 'bg-gray-300'
                }`}
              ></label>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <label htmlFor="hideHeader" className="text-sm font-medium">
              Hide Document Header
            </label>
            <div className="relative inline-block w-10 mr-2 align-middle select-none">
              <input 
                type="checkbox" 
                id="hideHeader" 
                checked={focusModeOptions.hideHeader}
                onChange={() => handleToggleOption('hideHeader')}
                className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
              />
              <label 
                htmlFor="hideHeader" 
                className={`toggle-label block overflow-hidden h-6 rounded-full cursor-pointer ${
                  focusModeOptions.hideHeader ? 'bg-blue-500' : 'bg-gray-300'
                }`}
              ></label>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <label htmlFor="dimUI" className="text-sm font-medium">
              Dim UI Elements
            </label>
            <div className="relative inline-block w-10 mr-2 align-middle select-none">
              <input 
                type="checkbox" 
                id="dimUI" 
                checked={focusModeOptions.dimUI}
                onChange={() => handleToggleOption('dimUI')}
                className="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
              />
              <label 
                htmlFor="dimUI" 
                className={`toggle-label block overflow-hidden h-6 rounded-full cursor-pointer ${
                  focusModeOptions.dimUI ? 'bg-blue-500' : 'bg-gray-300'
                }`}
              ></label>
            </div>
          </div>
        </div>

        <div className="mt-6 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
          >
            Done
          </button>
        </div>
      </div>
    </div>
  );
};

export default FocusModeSettings;
