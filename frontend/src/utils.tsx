const BASE_URLS = [
    'http://localhost:8001',
    'http://localhost:8000',
    import.meta.env.VITE_SECRET_SERVER + ":8001",
    import.meta.env.VITE_SECRET_SERVER + ":8000",
  ];
  
  export async function fetchWithFallback(path: string, options: RequestInit = {}) {
    const API_KEY = import.meta.env.VITE_API_TOKEN;
  
    const headers = {
      'Authorization': `Bearer ${API_KEY}`,
      'Content-Type': 'application/json',
      ...(options.headers || {}),
    };
  
    for (const baseUrl of BASE_URLS) {
      console.info(`[fetchWithFallback] Trying: ${baseUrl}${path}`);
      try {
        const res = await fetch(`${baseUrl}${path}`, {
          ...options,
          headers,
        });
  
        console.info(`[fetchWithFallback] Got response: ${res.status} from ${baseUrl}${path}`);
  
        if (!res.ok) {
          let responseText = '';
          try {
            responseText = await res.text();
            console.warn(`[fetchWithFallback] Failed status ${res.status}. Response: ${responseText}`);
          } catch (err) {
            console.warn(`[fetchWithFallback] Failed to read error body`);
          }
          continue;
        }
  
        return res; // 🚨 Return the actual Response object, NOT parsed JSON
      } catch (err) {
        console.error(`[fetchWithFallback] Network error on ${baseUrl}${path}:`, err);
        // Try next server
      }
    }
  
    console.error('[fetchWithFallback] All fetch attempts failed.');
    throw new Error('All fetch attempts failed.');
  }
  
  
  