require_relative '../util'

class Operation
  attr_reader :col, :row
end

class Rect < Operation
  def initialize(cols, rows)
    super()
    @col = cols
    @row = rows
  end
end

class RotateRow < Operation
  def initialize(row, cols)
    super()
    @row = row
    @col = cols
  end
end

class RotateColumn < Operation
  def initialize(col, rows)
    super()
    @col = col
    @row = rows
  end
end

class Day8
  def initialize
    @input = Util.read_input(8).split("\n").map do |row|
      match = row.match(/rect (\d+)x(\d+)/) { |m| Rect.new(m[1].to_i, m[2].to_i) }
      match ||= row.match(/rotate row y=(\d+) by (\d+)/) { |m| RotateRow.new(m[1].to_i, m[2].to_i) }
      match ||= row.match(/rotate column x=(\d+) by (\d+)/) { |m| RotateColumn.new(m[1].to_i, m[2].to_i) }
      match
    end
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def print_screen(screen)
    screen.each do |row|
      row.each do |col|
        print(col == 1 ? '#' : '.')
      end

      print("\n")
    end

    nil
  end

  def calculate_screen
    screen_x = 50
    screen_y = 6
    screen = Array.new(screen_y) { |_| Array.new(screen_x, 0) }

    @input.each do |op|
      case op
      when Rect
        (0..op.col - 1).to_a.product((0..op.row - 1).to_a).each do |(x, y)|
          screen[y][x] = 1
        end
      when RotateRow
        screen[op.row] = screen[op.row].rotate(-op.col)
      when RotateColumn
        screen.transpose[op.col].rotate(-op.row).each_with_index do |val, y|
          screen[y][op.col] = val
        end
      end
    end

    screen
  end

  def problem_one
    calculate_screen.map(&:sum).sum
  end

  def problem_two
    print_screen(calculate_screen)
  end
end

Day8.new.main
