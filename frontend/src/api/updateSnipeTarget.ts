type SnipeParams = {
    target_id: number;
    target_name?: string;
    sol_amount?: number;
    slippage?: number;
    priority_fee?: number;
    is_active?: boolean;
    deactive_on_snipe?: boolean;
  };
  
  export async function updateSnipeTarget(params: SnipeParams) {
    const response = await fetch('http://localhost:8001/api/v1/snipe', {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(params),
    });
  
    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`Snipe request failed: ${response.status} ${response.statusText} - ${errorBody}`);
    }
  
    return response.json();
  }
  