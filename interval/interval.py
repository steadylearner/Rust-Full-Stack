import time, threading

StartTime = time.time()


def post() :
    print('The exactly same automated post! -> time : {:.1f}s'.format(time.time() - StartTime))


class setInterval :
    def __init__(self, interval, post) :
        self.interval = interval
        self.post = post
        self.stopEvent = threading.Event()
        thread = threading.Thread(target=self.__setInterval)
        thread.start()

    def __setInterval(self) :
        nextTime = time.time() + self.interval
        while not self.stopEvent.wait(nextTime - time.time()) :
            nextTime += self.interval
            self.post()

    def cancel(self) :
        self.stopEvent.set()


inter = setInterval(1, post) # Once a week.(It will stay for a week.)
print('First post! -> time : {:.1f}s'.format(time.time() - StartTime))

# Stop automtaed post after 8  weeks.
t=threading.Timer(7, inter.cancel)

t.start()
