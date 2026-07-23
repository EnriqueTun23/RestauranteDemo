import {
  NavLink,
  Outlet,
} from "react-router-dom";

import {
  ClipboardList,
  MonitorSmartphone,
  QrCode,
  Settings,
  UtensilsCrossed,
} from "lucide-react";

const menu = [
  {
    to: "/orders",
    label: "Pedidos",
    icon: ClipboardList,
  },
  {
    to: "/devices",
    label: "Dispositivos",
    icon: MonitorSmartphone,
  },
  {
    to: "/registration",
    label: "Código QR",
    icon: QrCode,
  },
  {
    to: "/settings",
    label: "Configuración",
    icon: Settings,
  },
];

export function AppLayout() {
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar__brand">
          <div className="sidebar__logo">
            <UtensilsCrossed size={22} />
          </div>

          <div>
            <strong>Restaurante</strong>
            <span>Administrador</span>
          </div>
        </div>

        <nav className="sidebar__navigation">
          {menu.map((item) => {
            const Icon = item.icon;

            return (
              <NavLink
                key={item.to}
                to={item.to}
                className={({ isActive }) =>
                  isActive
                    ? "sidebar-link sidebar-link--active"
                    : "sidebar-link"
                }
              >
                <Icon size={19} />
                <span>{item.label}</span>
              </NavLink>
            );
          })}
        </nav>

        <div className="sidebar__footer">
          <span className="connection-dot" />
          Servidor local
        </div>
      </aside>

      <main className="main-content">
        <Outlet />
      </main>
    </div>
  );
}