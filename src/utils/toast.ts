import { toast } from 'react-hot-toast';

/**
 * Utility function for showing toast notifications
 * Wraps react-hot-toast for consistent usage across the app
 */
export const showToast = {
  success: (message: string) => toast.success(message),
  error: (message: string) => toast.error(message),
  loading: (message: string) => toast.loading(message),
  info: (message: string) => toast(message),
  dismiss: (toastId?: string) => toast.dismiss(toastId),
};

// Default export for backward compatibility
export default showToast;