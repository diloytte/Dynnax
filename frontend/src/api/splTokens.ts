export type TokenMetadata = {
    mint: string;
    standard: string;
    name: string;
    symbol: string;
    logo: string;
    decimals: string;
    metaplex: {
      metadataUri: string;
      masterEdition: boolean;
      isMutable: boolean;
      sellerFeeBasisPoints: number;
      updateAuthority: string;
      primarySaleHappened: number;
    };
    fullyDilutedValue: string;
    totalSupply: string;
    totalSupplyFormatted: string;
    links: {
      medium?: string;
      telegram?: string;
      twitter?: string;
      website?: string;
      github?: string;
      reddit?: string;
      moralis?: string;
    };
    description: string | null;
  };
  


  export const fetchTokenMetadata = async (ca: string): Promise<TokenMetadata | undefined> => {
    try {
      const res = await fetch(
        `https://solana-gateway.moralis.io/token/mainnet/${ca}/metadata`,
        {
          method: 'GET',
          headers: {
            accept: 'application/json',
            'X-API-Key': import.meta.env.VITE_MORALIS_API_KEY,
          },
        }
      );
  
      if (!res.ok) throw new Error(`Error: ${res.status}`);
  
      const data: TokenMetadata = await res.json();
  
      console.log('Response:', data);
      return data;
    } catch (err) {
      console.error('Fetch error:', err);
    }
  };
  