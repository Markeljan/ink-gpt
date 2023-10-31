
import { useChat, type Message } from 'ai/react'

import { ChatList } from './chat-list'
import { ChatPanel } from './chat-panel'
import { cn } from './utils'

export interface ChatProps extends React.ComponentProps<'div'> {
    initialMessages?: Message[]
}

export function Chat({ initialMessages, className }: ChatProps) {
    const { messages, append, reload, stop, isLoading, input, setInput } =
        useChat({
            initialMessages,
            api: 'http://127.0.0.1:8000/api/chat',
            headers: {
                'Content-Type': 'application/json'
            }
            
        })

    return (
        <>
            <div className={cn('pb-[200px] pt-4 md:pt-10', className)}>
                {messages.length ? (
                    <>
                        <ChatList messages={messages} />
                    </>
                ) : <></>
                }
            </div>
            <ChatPanel
                isLoading={isLoading}
                stop={stop}
                append={append}
                reload={reload}
                messages={messages}
                input={input}
                setInput={setInput}
            />
        </>
    )
}