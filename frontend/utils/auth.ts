import { config } from "@/config/config";

// Types
export interface User {
  id: number;
  username: string;
  fullname: string;
  whatsapp: string;
  role: string;
}

// Check if user is authenticated
export const isAuthenticated = (): boolean => {
  return !!localStorage.getItem('accessToken');
};

// Get current user from localStorage
export const getCurrentUser = (): User | null => {
  const userStr = localStorage.getItem('user');
  if (!userStr) return null;
  
  try {
    return JSON.parse(userStr);
  } catch (e) {
    return null;
  }
};

// Handle token refresh
export const refreshToken = async (): Promise<boolean> => {
  const refreshToken = localStorage.getItem('refreshToken');
  
  if (!refreshToken) {
    return false;
  }
  
  try {
    const response = await fetch(`${config.backendUrl}${config.apiEndpoints.refreshToken}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ refresh_token: refreshToken }),
    });
    
    if (!response.ok) {
      throw new Error('Failed to refresh token');
    }
    
    const accessToken = response.headers.get('X-Access-Token');
    const data = await response.json();
    
    if (accessToken) {
      localStorage.setItem('accessToken', accessToken);
      localStorage.setItem('user', JSON.stringify(data.user));
      return true;
    }
    
    return false;
  } catch (e) {
    return false;
  }
};

// Logout function
export const logout = async (): Promise<void> => {
  const refreshToken = localStorage.getItem('refreshToken');
  
  if (refreshToken) {
    try {
      await fetch(`${config.backendUrl}${config.apiEndpoints.logout}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ refresh_token: refreshToken }),
      });
    } catch (e) {
      // Ignore errors during logout
    }
  }
  
  // Clear local storage regardless of API response
  localStorage.removeItem('accessToken');
  localStorage.removeItem('refreshToken');
  localStorage.removeItem('user');
  
  // Redirect to login page
  window.location.href = '/auth#login';
};

// API request with authentication
export const authenticatedFetch = async (
  url: string, 
  options: RequestInit = {}
): Promise<Response> => {
  const accessToken = localStorage.getItem('accessToken');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }
  
  // Add authorization header
  const headers = {
    ...options.headers,
    'Authorization': `Bearer ${accessToken}`,
    'Content-Type': 'application/json',
  };
  
  try {
    const response = await fetch(url, {
      ...options,
      headers,
    });
    
    // If unauthorized, try to refresh token
    if (response.status === 401) {
      const refreshed = await refreshToken();
      
      if (refreshed) {
        // Retry with new token
        const newAccessToken = localStorage.getItem('accessToken');
        return fetch(url, {
          ...options,
          headers: {
            ...options.headers,
            'Authorization': `Bearer ${newAccessToken}`,
            'Content-Type': 'application/json',
          },
        });
      } else {
        // If refresh failed, logout
        await logout();
        throw new Error('Authentication failed');
      }
    }
    
    return response;
  } catch (error) {
    throw error;
  }
};
