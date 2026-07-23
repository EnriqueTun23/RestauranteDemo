import {
  Plus,
  RefreshCw,
} from "lucide-react";

import {
  useCallback,
  useEffect,
  useState,
} from "react";

import { useNavigate } from "react-router-dom";

import { Button } from "../../../components/ui/Button";

import {
  deleteOrder,
  listOrders,
} from "../orders.api";

import {
  OrderTable,
} from "../components/OrderTable";

import type {
  Order,
} from "../orders.types";

export function OrdersPage() {
  const navigate = useNavigate();

  const [orders, setOrders] =
    useState<Order[]>([]);

  const [loading, setLoading] =
    useState(true);

  const [error, setError] =
    useState<string | null>(null);

  const loadOrders = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const result = await listOrders();
      setOrders(result);
    } catch (requestError) {
      setError(String(requestError));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadOrders();
  }, [loadOrders]);

  async function handleDelete(order: Order) {
    const accepted = window.confirm(
      `¿Eliminar el pedido de la mesa ${order.tableNumber}?`,
    );

    if (!accepted) {
      return;
    }

    try {
      await deleteOrder(order.id);
      await loadOrders();
    } catch (requestError) {
      setError(String(requestError));
    }
  }

  return (
    <section>
      <div className="page-heading">
        <div>
          <h1>Pedidos</h1>
          <p>
            Consulta y administra los pedidos
            del restaurante.
          </p>
        </div>

        <div className="page-heading__actions">
          <Button
            variant="secondary"
            onClick={() => void loadOrders()}
          >
            <RefreshCw size={18} />
            Actualizar
          </Button>

          <Button
            onClick={() =>
              navigate("/orders/new")
            }
          >
            <Plus size={18} />
            Nuevo pedido
          </Button>
        </div>
      </div>

      {error && (
        <div className="alert alert--error">
          {error}
        </div>
      )}

      <div className="card">
        {loading ? (
          <div className="loading-state">
            Cargando pedidos...
          </div>
        ) : (
          <OrderTable
            orders={orders}
            onEdit={(order) =>
              navigate(
                `/orders/${order.id}/edit`,
              )
            }
            onDelete={(order) =>
              void handleDelete(order)
            }
          />
        )}
      </div>
    </section>
  );
}