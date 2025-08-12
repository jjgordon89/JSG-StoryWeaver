import React, { useState } from 'react';
import { AsyncButton } from '../ui/AsyncButton';
import { LoadingState, useLoadingState } from '../ui/LoadingState';
import { SafeComponent } from '../ui/SafeComponent';
import { useErrorHandler } from '../../hooks/useErrorHandler';
import { useGlobalError } from '../providers/ErrorProvider';
import { showSuccessToast } from '../providers/ToastProvider';
import { AlertCircle, CheckCircle, Zap } from 'lucide-react';

// Example component that might throw an error
const ProblematicComponent: React.FC<{ shouldError: boolean }> = ({ shouldError }) => {
  if (shouldError) {
    throw new Error('This is a simulated component error!');
  }
  
  return (
    <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
      <div className="flex items-center gap-2">
        <CheckCircle className="w-5 h-5 text-green-600" />
        <span className="text-green-800">Component rendered successfully!</span>
      </div>
    </div>
  );
};

// Example of async operation that might fail
const simulateAsyncOperation = async (shouldFail: boolean = false): Promise<string> => {
  await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate delay
  
  if (shouldFail) {
    throw new Error('Simulated async operation failed!');
  }
  
  return 'Async operation completed successfully!';
};

export const ErrorHandlingExample: React.FC = () => {
  const [componentShouldError, setComponentShouldError] = useState(false);
  const [asyncResult, setAsyncResult] = useState<string | null>(null);
  const [fetchData, setFetchData] = useState<string[]>([]);
  
  const { handleError, handleWarning } = useErrorHandler();
  const { addError, addWarning, addInfo } = useGlobalError();
  const { isLoading, error, startLoading, stopLoading, setError, reset } = useLoadingState();

  const handleAsyncSuccess = async () => {
    try {
      const result = await simulateAsyncOperation(false);
      setAsyncResult(result);
      showSuccessToast('Operation completed successfully!');
    } catch (err) {
      handleError(err, { component: 'ErrorHandlingExample', operation: 'success' });
    }
  };

  const handleAsyncFailure = async () => {
    try {
      await simulateAsyncOperation(true);
    } catch (err) {
      // This will be handled by AsyncButton automatically
      throw err;
    }
  };

  const handleManualError = () => {
    addError('This is a manually triggered error', { source: 'manual', timestamp: Date.now() });
  };

  const handleManualWarning = () => {
    addWarning('This is a warning message', { source: 'manual' });
  };

  const handleManualInfo = () => {
    addInfo('This is an info message', { source: 'manual' });
  };

  const simulateDataFetch = async () => {
    startLoading();
    try {
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      if (Math.random() > 0.7) {
        throw new Error('Random fetch failure!');
      }
      
      setFetchData(['Item 1', 'Item 2', 'Item 3']);
      stopLoading();
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-6 space-y-8">
      <div className="text-center">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Error Handling Examples
        </h1>
        <p className="text-gray-600">
          Demonstration of comprehensive error handling patterns in StoryWeaver
        </p>
      </div>

      {/* Error Boundary Example */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-gray-800 flex items-center gap-2">
          <AlertCircle className="w-5 h-5" />
          Error Boundary Example
        </h2>
        <div className="space-y-4">
          <div className="flex gap-4">
            <button
              onClick={() => setComponentShouldError(false)}
              className={`px-4 py-2 rounded-md transition-colors ${
                !componentShouldError
                  ? 'bg-green-600 text-white'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Show Working Component
            </button>
            <button
              onClick={() => setComponentShouldError(true)}
              className={`px-4 py-2 rounded-md transition-colors ${
                componentShouldError
                  ? 'bg-red-600 text-white'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              Trigger Component Error
            </button>
          </div>
          
          <SafeComponent
            errorTitle="Component Error Example"
            errorMessage="This demonstrates how SafeComponent handles rendering errors."
          >
            <ProblematicComponent shouldError={componentShouldError} />
          </SafeComponent>
        </div>
      </section>

      {/* Async Button Examples */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-gray-800 flex items-center gap-2">
          <Zap className="w-5 h-5" />
          Async Button Examples
        </h2>
        <div className="flex flex-wrap gap-4">
          <AsyncButton
            onClick={handleAsyncSuccess}
            variant="primary"
            loadingText="Processing..."
          >
            Successful Async Operation
          </AsyncButton>
          
          <AsyncButton
            onClick={handleAsyncFailure}
            variant="danger"
            loadingText="Failing..."
          >
            Failing Async Operation
          </AsyncButton>
        </div>
        
        {asyncResult && (
          <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
            <p className="text-blue-800">{asyncResult}</p>
          </div>
        )}
      </section>

      {/* Loading State Example */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-gray-800">
          Loading State Example
        </h2>
        <div className="space-y-4">
          <div className="flex gap-4">
            <button
              onClick={simulateDataFetch}
              disabled={isLoading}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Fetch Data
            </button>
            <button
              onClick={reset}
              className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700"
            >
              Reset
            </button>
          </div>
          
          <LoadingState
            isLoading={isLoading}
            error={error}
            isEmpty={!isLoading && !error && fetchData.length === 0}
            loadingText="Fetching data..."
            emptyText="No data to display. Click 'Fetch Data' to load some."
            retryAction={simulateDataFetch}
            className="min-h-[200px] border border-gray-200 rounded-lg"
          >
            <div className="p-4">
              <h3 className="font-medium text-gray-900 mb-2">Fetched Data:</h3>
              <ul className="space-y-1">
                {fetchData.map((item, index) => (
                  <li key={index} className="text-gray-700">
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          </LoadingState>
        </div>
      </section>

      {/* Manual Error Reporting */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-gray-800">
          Manual Error Reporting
        </h2>
        <div className="flex flex-wrap gap-4">
          <button
            onClick={handleManualError}
            className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700"
          >
            Trigger Error Toast
          </button>
          <button
            onClick={handleManualWarning}
            className="px-4 py-2 bg-yellow-600 text-white rounded-md hover:bg-yellow-700"
          >
            Trigger Warning Toast
          </button>
          <button
            onClick={handleManualInfo}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            Trigger Info Toast
          </button>
        </div>
      </section>
    </div>
  );
};

export default ErrorHandlingExample;