<?php

namespace App\Jobs;

use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Queue\Queueable;
use Illuminate\Support\Facades\Log;
use Kreait\Firebase\Contract\Messaging;
use Kreait\Firebase\Messaging\CloudMessage;
use Kreait\Firebase\Messaging\Notification;

class FCMSendMultiNotiJob implements ShouldQueue
{
    use Queueable;

    /**
     * Create a new job instance.
     *
     * @param  array<string>  $tokens  FCM device tokens (max 500 per Firebase limit)
     */
    public function __construct(
        public string $title,
        public string $description,
        public array $tokens,
        public array $data = []
    ) {
    }

    /**
     * Execute the job.
     */
    public function handle(Messaging $messaging): void
    {
        if (empty($this->tokens)) {
            Log::warning(__METHOD__ . ': No tokens provided');
            return;
        }

        $message = CloudMessage::new()
            ->withNotification(Notification::create($this->title, $this->description))
            ->withData($this->data);

        $report = $messaging->sendMulticast($message, $this->tokens);

        Log::info(__METHOD__, [
            'successes' => $report->successes()->count(),
            'failures' => $report->failures()->count(),
        ]);

        if ($report->hasFailures()) {
            foreach ($report->failures()->getItems() as $failure) {
                Log::warning(__METHOD__ . ': FCM multicast failure', [
                    'token' => $failure->target()->value(),
                    'error' => $failure->error()?->getMessage(),
                ]);
            }
        }
    }
}
