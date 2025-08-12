import React from 'react';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

interface ToastProviderProps {
  children: React.ReactNode;
}

export const ToastProvider: React.FC<ToastProviderProps> = ({ children }) => {
  return (
    <>
      {children}
      <ToastContainer
        position="top-right"
        autoClose={5000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="light"
        toastClassName="bg-white border border-gray-200 shadow-lg"
        bodyClassName="text-gray-800"
        progressClassName="bg-blue-500"
      />
    </>
  );
};

// Utility functions for consistent toast usage
export const showSuccessToast = (message: string) => {
  toast.success(message, {
    className: 'bg-green-50 border-green-200',
    progressClassName: 'bg-green-500',
  });
};

export const showErrorToast = (message: string) => {
  toast.error(message, {
    className: 'bg-red-50 border-red-200',
    progressClassName: 'bg-red-500',
  });
};

export const showWarningToast = (message: string) => {
  toast.warn(message, {
    className: 'bg-yellow-50 border-yellow-200',
    progressClassName: 'bg-yellow-500',
  });
};

export const showInfoToast = (message: string) => {
  toast.info(message, {
    className: 'bg-blue-50 border-blue-200',
    progressClassName: 'bg-blue-500',
  });
};

export default ToastProvider;