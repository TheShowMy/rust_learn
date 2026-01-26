import { useState, useEffect, useCallback, useRef } from 'react';
import type { ChatMessage } from '../types';

export const useChat = (username: string | null, room: string | null) => {
    const [messages, setMessages] = useState<ChatMessage[]>([]);
    const [isConnected, setIsConnected] = useState(false);
    const ws = useRef<WebSocket | null>(null);

    const connect = useCallback(() => {
        if (!username || !room) return;

        // Use absolute URL or relative if served by same server
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const host = window.location.hostname === 'localhost' ? 'localhost:3000' : window.location.host;
        const socketUrl = `${protocol}//${host}/ws`;

        const socket = new WebSocket(socketUrl);

        socket.onopen = () => {
            console.log('Connected to chat server');
            setIsConnected(true);

            // Send join message
            const joinMsg: ChatMessage = {
                room,
                username,
                timestamp: Math.floor(Date.now() / 1000),
                message: "join"
            };
            socket.send(JSON.stringify(joinMsg));
        };

        socket.onmessage = (event) => {
            try {
                const msg: ChatMessage = JSON.parse(event.data);
                setMessages((prev) => [...prev, msg]);
            } catch (err) {
                console.error('Failed to parse message', err);
            }
        };

        socket.onclose = () => {
            console.log('Disconnected from chat server');
            setIsConnected(false);
            // Optional: auto-reconnect logic
        };

        socket.onerror = (error) => {
            console.error('WebSocket error', error);
        };

        ws.current = socket;
    }, [username, room]);

    const sendMessage = useCallback((text: string) => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN && username && room) {
            const chatMsg: ChatMessage = {
                room,
                username,
                timestamp: Math.floor(Date.now() / 1000),
                message: { message: text }
            };
            ws.current.send(JSON.stringify(chatMsg));
        }
    }, [username, room]);

    const disconnect = useCallback(() => {
        if (ws.current) {
            ws.current.close();
            ws.current = null;
        }
    }, []);

    useEffect(() => {
        if (username && room) {
            connect();
        }
        return () => {
            disconnect();
        };
    }, [username, room, connect, disconnect]);

    return { messages, isConnected, sendMessage, setMessages };
};
