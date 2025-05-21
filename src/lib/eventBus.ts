import mitt from "mitt"
import {INode} from "@/type/node.ts";
import {ILabel} from "@/type/label.ts";
import {ITunnel} from "@/type/tunnel.ts";

type Events = {
    "tunnel_update": void | { label: number },
    "tunnel_contextmenu": { label: ILabel[] | null, item: ITunnel },
    "tunnel_add_update": void | { label: number },
    "tunnel_add_tcp_udp": { protocol: string, node_id: number, node_name: string, node_ip: string, node_port: number },
    "tunnel_add_http": { protocol: string, node_id: number, node_name: string, node_ip: string, node_port: number },
    "tunnel_edit": ITunnel,
    "node_update": void | { label: number },
    "node_contextmenu": { label: ILabel[] | null, item: INode },
    "label_manage_update": { type: "tunnel" | "node" },
}

export const emitter = mitt<Events>()