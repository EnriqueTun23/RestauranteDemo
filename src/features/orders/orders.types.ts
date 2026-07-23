export type OrderStatus =
  | "pending"
  | "preparing"
  | "ready"
  | "delivered"
  | "cancelled";

export interface OrderItem {
  id: string;
  orderId: string;
  productName: string;
  quantity: number;
  unitPriceCents: number;
  notes?: string | null;
}

export interface Order {
  id: string;
  tableNumber: string;
  customerName?: string | null;
  notes?: string | null;
  status: OrderStatus;
  totalCents: number;
  createdAt: string;
  updatedAt: string;
  items: OrderItem[];
}

export interface CreateOrderItemInput {
  productName: string;
  quantity: number;
  unitPriceCents: number;
  notes?: string;
}

export interface CreateOrderInput {
  tableNumber: string;
  customerName?: string;
  notes?: string;
  items: CreateOrderItemInput[];
}