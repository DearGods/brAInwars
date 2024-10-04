import {
  PublicKey,
  SendOptions,
  Transaction,
  TransactionSignature,
} from "@solana/web3.js";

enum WalletType {
  PHANTOM = "PHANTOM", // window.solana
  SOLFLARE = "SOLFLARE", // window.solflare
}

export class Wallet {
  wallet: any;
  walletType: WalletType | undefined = undefined;

  constructor(wallet: any) {
    this.wallet = wallet;
    this.walletType = this.getWalletType(wallet);
  }

  publicKey(): PublicKey {
    return this.wallet.publicKey;
  }

  isConnected(): boolean {
    return this.wallet.isConnected;
  }

  getWalletType(wallet: any): WalletType | undefined {
    if (wallet?.isPhantom) {
      return WalletType.PHANTOM;
    }
    if (wallet?.isSolflare) {
      return WalletType.SOLFLARE;
    }
    return undefined;
  }

  async signTransaction(transaction: Transaction): Promise<Transaction> {
    return await this.wallet.signTransaction(transaction);
  }

  async signAllTransactions(
    transactions: Transaction[],
  ): Promise<Transaction[]> {
    return await this.wallet.signAllTransactions(transactions);
  }

  async signAndSendTransaction(
    transaction: Transaction,
    options?: SendOptions,
  ): Promise<{ signature: TransactionSignature }> {
    return await this.wallet.signAndSendTransaction(transaction, options);
  }

  async signMessage(message: Uint8Array): Promise<{ signature: Uint8Array }> {
    return await this.wallet.signMessage(message);
  }

  async connect(): Promise<void> {
    // logger.debug("Wallet: connect", { wallet: this.wallet });
    return await this.wallet.connect();
  }

  async disconnect(): Promise<void> {
    return await this.wallet.disconnect();
  }
}

/*
export interface WalletContextType {
    wallet: Wallet | undefined;
    setWallet: (wallet: any) => void;
    supportedWallets: Wallets;
    connected: boolean;
}

export const Context = createContext<WalletContextType>(
    {} as WalletContextType
);

export const useWallet = (): WalletContextType => {
    return useContext(Context);
};

type Wallets = {
    Phantom: any;
    SolFlare: any;
};

type WalletProviderProps = {
    wallets: Wallets;
};

export const WalletProvider: FC<PropsWithChildren<WalletProviderProps>> = ({
                                                                               children,
                                                                               wallets,
                                                                           }) => {
    const [wallet, _setWallet] = useState<Wallet>();
    const [supportedWallets, _setSupportedWallets] = useState<Wallets>(wallets);
    const [connected, setConnected] = useState<boolean>(false);

    function setWallet(wallet: any) {
        logger.debug("WalletProvider: setWallet", { wallet });
        if (wallet === undefined || wallet === null) {
            alert("Wallet is undefined or null, please try again");
            return;
        }
        _setWallet(new Wallet(wallet = window.solana));
        wallet.on("connect", () => {
            logger.debug("WalletProvider: wallet.on(connect)", { wallet });
            setConnected(true);
        });
        wallet.on("disconnect", () => {
            logger.debug("WalletProvider: wallet.on(disconnect)", { wallet });
            setConnected(false);
        });
    }

    useEffect(() => {
        logger.debug("WalletProvider: useEffect: wallet changed", {
            wallet,
            connected,
        });
    }, [wallets, wallet]);

    return (
        <Context.Provider
            value={{
        wallet,
            setWallet,
            supportedWallets,
            connected,
    }}
>
    {children}
    </Context.Provider>
);
};



import {
    BlockheightBasedTransactionConfirmationStrategy,
    Connection,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import { Wallet } from "../context/WalletContext";
import { TxnResult } from "./helpers";

export async function processTransactionWithWallet(
    instructions: TransactionInstruction[],
    connection: Connection,
    wallet: Wallet
): Promise<TxnResult> {
    const tx = new Transaction();
    instructions.map((i) => tx.add(i));
    const blockStats = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockStats.blockhash;
    tx.feePayer = wallet?.publicKey();
    const signedTx = await wallet.signTransaction(tx);
    const sig = await connection.sendRawTransaction(signedTx.serialize(), {
        maxRetries: 3,
        preflightCommitment: "confirmed",
        skipPreflight: true,
    });
    const strategy: BlockheightBasedTransactionConfirmationStrategy = {
        signature: sig,
        blockhash: blockStats.blockhash,
        lastValidBlockHeight: blockStats.lastValidBlockHeight,
    };
    const result = await connection.confirmTransaction(strategy, "processed");
    return {
        Signature: sig,
        SignatureResult: result.value,
    };
}
*/
