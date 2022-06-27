import { WalletConnection } from "near-api-js";
import { useEffect, useMemo, useState } from "react";
import { useNearUser, useNearWallet } from "react-near";
import { LoadingLottie } from "../assets/animations";
import { Button } from "../components/shared/button";
import { If } from "../components/if";
import { NearLogo } from "../components/nearLogo";
import { contractName } from "../env/contract";
import { useGreetingStore } from "../stores/greeting";

/**
 * @route - '/'
 * @description - This is the landing page for the near application
 * @name Index
 */
export function Index() {
  const [newGreetingForm, setNewGreetingForm] = useState("");
  const wallet = useNearWallet();
  const user = useNearUser(contractName);
  const {
    initializeContract,
    greeting,
    setNewGreeting,
    fetchGreeting,
    isUpdating,
  } = useGreetingStore();

  useEffect(() => {
    if (user.account && user.isConnected)
      initializeContract(
        wallet as WalletConnection,
        user.account?.accountId as string
      ).then(() => {
        fetchGreeting(user.account?.accountId as string);
      });
  }, [user.isConnected]);

  const connectToNear = async () => {
    await wallet?.requestSignIn();
    await user.connect();
  };

  const walletConnected = useMemo(() => !!wallet?.isSignedIn(), [wallet]);

  const submit = async () => {
    if (isUpdating && user.account?.accountId) {
      return;
    }

    await setNewGreeting(newGreetingForm, user.account?.accountId as string);

    setNewGreetingForm("");
  };

  return (
    <main className="max-w-[1440px] bg-white min-w-[100vw] min-h-[100vh]">
      <div
        
      >

      </div>
      
    </main>
  );
}
