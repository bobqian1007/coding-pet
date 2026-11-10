import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./App.css";
import { Pet, type PetStatus } from "./components/Pet";

interface UnifiedEvent {
  event: PetStatus;
}

function App() {
  const [status, setStatus] = useState<PetStatus>("IDLE");

  useEffect(() => {
    let disposed = false;
    let unlisten: UnlistenFn | undefined;

    void listen<UnifiedEvent>("task-status-changed", ({ payload }) => {
      setStatus(payload.event);
    }).then((stopListening) => {
      if (disposed) {
        stopListening();
      } else {
        unlisten = stopListening;
      }
    });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  return (
    <main className="pet-window" data-tauri-drag-region>
      <Pet status={status} />
    </main>
  );
}

export default App;
