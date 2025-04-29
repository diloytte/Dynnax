const BASE_URLS = [
    'http://localhost:8001',
    'http://localhost:8000',
    import.meta.env.VITE_SECRET_SERVER,
  ];
  
export async function fetchWithFallback(path: string, options: RequestInit = {}) {
    const API_KEY = import.meta.env.VITE_API_TOKEN;
  
    const headers = {
      'Authorization': `Bearer ${API_KEY}`,
      'Content-Type': 'application/json',
      ...(options.headers || {}),
    };
  
    for (const baseUrl of BASE_URLS) {
      try {
        const res = await fetch(`${baseUrl}${path}`, {
          ...options,
          headers,
        });
  
        if (!res.ok) {
          console.warn(`Fetch failed on ${baseUrl}${path} with status: ${res.status}`);
          continue;
        }
  
        return res; // Success
      } catch (err) {
        console.warn(`Fetch error on ${baseUrl}${path}:`, err);
      }
    }
  
    console.error('All fetch attempts failed. All servers are down.');
    throw new Error('All fetch attempts failed.');
  }
  