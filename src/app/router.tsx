import {
  createBrowserRouter,
  Navigate,
} from "react-router-dom";

import {
  AppLayout,
} from "../components/layout/AppLayout";

import {
  OrdersPage,
} from "../features/orders/pages/OrdersPage";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      {
        index: true,
        element: (
          <Navigate
            to="/orders"
            replace
          />
        ),
      },
      {
        path: "orders",
        element: <OrdersPage />,
      },
      {
        path: "orders/new",
        element: (
          <div>Formulario de pedido</div>
        ),
      },
      {
        path: "orders/:orderId/edit",
        element: (
          <div>Editar pedido</div>
        ),
      },
      {
        path: "devices",
        element: (
          <div>Dispositivos</div>
        ),
      },
      {
        path: "registration",
        element: (
          <div>Registro por QR</div>
        ),
      },
      {
        path: "settings",
        element: (
          <div>Configuración</div>
        ),
      },
    ],
  },
]);