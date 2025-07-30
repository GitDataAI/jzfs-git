import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import "./style/root.css"
import AppRoutes from "./routes.tsx";
import {MantineProvider} from "@mantine/core";
import {Notifications} from "@mantine/notifications";
import '@mantine/core/styles.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
      <MantineProvider>
          <Notifications position="top-center" withinPortal style={{
              zIndex: 9999,
              width: 400,
              position: "fixed",
              top: "2rem",
              left: "50%",
              transform: "translateX(-50%)",
          }}/>
          <AppRoutes />
      </MantineProvider>
  </StrictMode>,
)
