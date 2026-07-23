import type {
  OrderStatus,
} from "../orders.types";

interface Props {
  status: OrderStatus;
}

const labels: Record<OrderStatus, string> = {
  pending: "Pendiente",
  preparing: "Preparando",
  ready: "Listo",
  delivered: "Entregado",
  cancelled: "Cancelado",
};

export function OrderStatusBadge({
  status,
}: Props) {
  return (
    <span
      className={`status-badge status-badge--${status}`}
    >
      {labels[status]}
    </span>
  );
}