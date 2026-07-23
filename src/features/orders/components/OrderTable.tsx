import {
  Pencil,
  Trash2,
} from "lucide-react";

import type {
  Order,
} from "../orders.types";

import {
  OrderStatusBadge,
} from "./OrderStatusBadge";

interface Props {
  orders: Order[];
  onEdit: (order: Order) => void;
  onDelete: (order: Order) => void;
}

function formatCurrency(cents: number) {
  return new Intl.NumberFormat("es-MX", {
    style: "currency",
    currency: "MXN",
  }).format(cents / 100);
}

export function OrderTable({
  orders,
  onEdit,
  onDelete,
}: Props) {
  if (orders.length === 0) {
    return (
      <div className="empty-state">
        <h3>No hay pedidos</h3>
        <p>
          Crea el primer pedido para comenzar.
        </p>
      </div>
    );
  }

  return (
    <div className="table-container">
      <table className="data-table">
        <thead>
          <tr>
            <th>Mesa</th>
            <th>Productos</th>
            <th>Cliente</th>
            <th>Estado</th>
            <th>Total</th>
            <th aria-label="Acciones" />
          </tr>
        </thead>

        <tbody>
          {orders.map((order) => (
            <tr key={order.id}>
              <td>
                <strong>
                  Mesa {order.tableNumber}
                </strong>
              </td>

              <td>
                {order.items.map((item) => (
                  <div key={item.id}>
                    {item.quantity} ×{" "}
                    {item.productName}
                  </div>
                ))}
              </td>

              <td>
                {order.customerName || "—"}
              </td>

              <td>
                <OrderStatusBadge
                  status={order.status}
                />
              </td>

              <td>
                {formatCurrency(
                  order.totalCents,
                )}
              </td>

              <td>
                <div className="table-actions">
                  <button
                    className="icon-button"
                    onClick={() => onEdit(order)}
                    aria-label="Editar pedido"
                  >
                    <Pencil size={17} />
                  </button>

                  <button
                    className="icon-button icon-button--danger"
                    onClick={() => onDelete(order)}
                    aria-label="Eliminar pedido"
                  >
                    <Trash2 size={17} />
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}