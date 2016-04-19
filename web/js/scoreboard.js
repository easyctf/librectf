var chart = c3.generate({
    data: {
        x: 'x',
        columns: [
            ['x', '2013-01-01', '2013-01-02', '2013-01-03', '2013-01-04', '2013-01-05', '2013-01-06'],
            ['Team Thomas', 30, 250, 380, 500, 620, 740],
        ]
    },
    axis: {
        x: {
            type: 'timeseries',
            tick: {
                format: '%Y-%m-%d'
            }
        }
    }
});
setTimeout(function() {
    chart.load({
        columns: [
            ['Team Charles', 100, 210, 320, 430, 540, 650]
        ]
    });
}, 1000);
setTimeout(function() {
    chart.load({
        columns: [
            ['Team Zach', 10, 120, 230, 340, 450, 560]
        ]
    });
}, 2000);
setTimeout(function() {
    chart.load({
        columns: [
            ['Team Michael', 0, 80, 190, 300, 410, 520]
        ]
    });
}, 3000);
setTimeout(function() {
    chart.load({
        columns: [
            ['Team James', 0, 100, 250, 300, 450, 600]
        ]
    });
}, 4000);
