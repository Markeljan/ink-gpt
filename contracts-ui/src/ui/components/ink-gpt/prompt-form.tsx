import { useEffect, useRef } from 'react'
import { UseChatHelpers } from 'ai/react'
import Textarea from 'react-textarea-autosize'
import { useEnterSubmit } from './use-enter-submit'
import { cn } from './utils'
import { ArrowSmRightIcon, HomeIcon } from '@heroicons/react/outline'


export interface PromptProps
    extends Pick<UseChatHelpers, 'input' | 'setInput'> {
    onSubmit: (value: string) => Promise<void>
    isLoading: boolean
}

export function PromptForm({
    onSubmit,
    input,
    setInput,
    isLoading
}: PromptProps) {
    const { formRef, onKeyDown } = useEnterSubmit()
    const inputRef = useRef<HTMLTextAreaElement>(null)

    useEffect(() => {
        if (inputRef.current) {
            inputRef.current.focus()
        }
    }, [])

    return (
        <form
            onSubmit={async e => {
                e.preventDefault()
                if (!input?.trim() || isLoading) {
                    return
                }
                setInput('')
                await onSubmit(input)
            }}
            ref={formRef}
        >
            <div className="relative flex max-h-60 w-full grow flex-col overflow-hidden bg-background px-8 sm:rounded-md sm:border sm:px-12">
                <button
                    onClick={() => {
                        location.href = '/ink-gpt'
                    }}
                    className={cn(
                        'absolute left-0 top-4 h-8 w-8 rounded-full bg-background p-0 sm:left-4'
                    )}
                >
                    <HomeIcon />
                    <span className="sr-only">New Chat</span>
                </button>
                <Textarea
                    ref={inputRef}
                    tabIndex={0}
                    onKeyDown={onKeyDown}
                    rows={1}
                    value={input}
                    onChange={e => setInput(e.target.value)}
                    placeholder="Send a message."
                    spellCheck={false}
                    className="min-h-[60px] w-full resize-none bg-transparent px-4 py-[1.3rem] focus-within:outline-none sm:text-sm"
                />
                <div className="absolute right-0 top-3 sm:right-4">
                    <button
                        type="submit"
                        disabled={isLoading || input === ''}
                    >
                        <ArrowSmRightIcon />
                    </button>
                </div>
            </div>
        </form >
    )
}