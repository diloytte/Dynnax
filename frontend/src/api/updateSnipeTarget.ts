import { fetchWithFallback } from "../utils";

type SnipeParams = {
  target_id?: number;
  target_name?: string;
  sol_amount?: number;
  slippage?: number;
  priority_fee?: number;
  is_active?: boolean;
  deactivate_on_snipe?: boolean;
  isTwitterTarget: boolean;
};

  
export async function updateSnipeTarget(params: SnipeParams) {
  const endpoint = params.isTwitterTarget ? "/api/v1/snipeX" : "/api/v1/snipe";

  const {
    isTwitterTarget,
    ...body
  } = params;

  const response = await fetchWithFallback(endpoint, {
    method: "PATCH",
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const errorBody = await response.text();
    throw new Error(`Snipe request failed: ${response.status} ${response.statusText} - ${errorBody}`);
  }

  return response.json();
}

  