export interface ITunnel {
    id: number,
    name: string,
    label: number,
    protocol: string,
    ip: string,
    port: number | undefined,
    remote_port: number | undefined,
    domain: string | undefined,
    node_id: number,
    node_name: string,
    node_ip: string,
    node_port: number,
    encrypt: boolean,
    compress: boolean,
    args: string,
    status: boolean,
}

export const tunnelProtocol = [
    { label: "TCP", value: "tcp" },
    { label: "UDP", value: "udp" },
    { label: "HTTP", value: "http" },
    { label: "HTTPS", value: "https" },
]